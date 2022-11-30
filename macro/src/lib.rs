mod node;

use crate::node::{Child, Children};
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, ItemImpl, Result, Token,
};

fn east_crate() -> proc_macro2::TokenStream {
    let found_crate = crate_name("east").expect("east is present in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident )
        }
    }
}

fn generate_children_to_string(
    component_type: proc_macro2::TokenStream,
    children: Vec<Child>,
) -> proc_macro2::TokenStream {
    let east_crate = east_crate();
    let output = Ident::new("__east_output", Span::mixed_site());

    let children = children.into_iter().map(|child| {
        match child {
            Child::Element(element) => {
                if let Some(html_tag) = element.html_tag() {
                    let attributes = element.attributes().into_iter().map(|attribute| {
                        let raw_name = format!("{}", attribute.name).replace("_", "-");

                        let name = proc_macro2::Literal::string(&raw_name);
                        let value = attribute.value;
                        quote! {
                            format!("{}=\"{}\"", #name, #east_crate::escape(&#value))
                        }
                    });

                    if element.children().is_empty() {
                        quote! {
                            #output.push_str("<");
                            #output.push_str(&[#html_tag, #(&#attributes),*].join(" "));
                            #output.push_str(">");

                            #output.push_str("</");
                            #output.push_str(&#html_tag);
                            #output.push_str(">");
                        }
                    } else {
                        let children = generate_children_to_string(component_type.clone(), element.children());

                        quote! {
                            #output.push_str("<");
                            #output.push_str(&[#html_tag, #(&#attributes),*].join(" "));
                            #output.push_str(">");

                            #output.push_str(&#children.0);

                            #output.push_str("</");
                            #output.push_str(&#html_tag);
                            #output.push_str(">");
                        }
                    }
                } else {
                    let tag = element.tag.clone();
                    let attributes = element.attributes().into_iter().map(|attribute| {
                        let name = attribute.name;
                        let value = attribute.value;
                        quote! { #name: #value }
                    });

                    if element.children().is_empty() {
                        quote! {
                            #output.push_str(&#east_crate::Render::<#component_type>::render(#tag {
                                #(#attributes),*
                            }).0);
                        }
                    } else {
                        let children = generate_children_to_string(component_type.clone(), element.children());

                        quote! {
                            #output.push_str(&#east_crate::RenderMulti::<#component_type>::render_multi(#tag {
                                #(#attributes),*
                            }, #children).0);
                        }
                    }
                }
            },
            Child::Expr(expr) => {
                quote! { #output.push_str(&#east_crate::Render::<#component_type>::render(#expr).0); }
            },
        }
    });

    quote! { {
        let mut #output = String::new();

        #(#children)*

        #east_crate::PreEscaped(#output)
    } }
}

fn generate_children_to_view(scope: Ident, children: Vec<Child>) -> proc_macro2::TokenStream {
    let east_crate = east_crate();

    let children = children.into_iter().map(|child| match child {
        Child::Element(element) => {
            if let Some(html_tag) = element.html_tag() {
                let attributes = element.attributes().into_iter().map(|attribute| {
                    let raw_name = format!("{}", attribute.name).replace("_", "-");

                    if raw_name.starts_with("on-") {
                        let mut raw_name = raw_name;
                        let name = proc_macro2::Literal::string(&raw_name.split_off(3));
                        let value = attribute.value;

                        quote! {
                            .on(#name, #value)
                        }
                    } else {
                        let name = proc_macro2::Literal::string(&raw_name);
                        let value = attribute.value;

                        quote! {
                            .dyn_attr(#name, { let #scope = #scope.clone(); move || #value })
                        }
                    }
                });

                let children = if element.children().is_empty() {
                    quote! {}
                } else {
                    let children = generate_children_to_view(scope.clone(), element.children());
                    quote! {
                        .dyn_c({ let #scope = #scope.clone(); move || #children })
                    }
                };

                let html_tag = proc_macro2::Literal::string(&html_tag);
                quote! {
                    #east_crate::builder::tag(#html_tag)
                    #(#attributes)*
                    #children
                    .view(#scope)
                }
            } else {
                let tag = element.tag.clone();
                let attributes = element.attributes().into_iter().map(|attribute| {
                    let name = attribute.name;
                    let value = attribute.value;
                    quote! { #name: #value }
                });

                if element.children().is_empty() {
                    quote! {
                        #east_crate::RenderDyn::render_dyn(#tag {
                            #(#attributes),*
                        }, #scope.clone())
                    }
                } else {
                    panic!("Dynamic element tags do not support children.");
                }
            }
        }
        Child::Expr(expr) => {
            quote! { #east_crate::RenderDyn::render_dyn(#expr, #scope) }
        }
    });

    quote! { {
        #east_crate::builder::fragment([
            #(#children),*
        ])
    } }
}

#[derive(Clone, Debug)]
struct Render {
    children: Children,
}

impl Parse for Render {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            children: input.parse()?,
        })
    }
}

/// Render a static markup without any dynamic component.
#[proc_macro]
pub fn render(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let east_crate = east_crate();
    let view = parse_macro_input!(input as Render);
    let component_type = quote! { #east_crate::NoComponent };

    generate_children_to_string(component_type, view.children.0.into_iter().collect()).into()
}

#[derive(Clone, Debug)]
struct RenderWithComponent {
    component_type: syn::Type,
    _comma_token: Token![,],
    _brace_token: syn::token::Brace,
    children: Children,
}

impl Parse for RenderWithComponent {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            component_type: input.parse()?,
            _comma_token: input.parse()?,
            _brace_token: braced!(content in input),
            children: content.parse()?,
        })
    }
}

/// Render a static markup with given dynamic component type.
#[proc_macro]
pub fn render_with_component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let view_with_component = parse_macro_input!(input as RenderWithComponent);
    let component_type = view_with_component.component_type;

    generate_children_to_string(
        quote! { #component_type },
        view_with_component.children.0.into_iter().collect(),
    )
    .into()
}

#[derive(Clone, Debug)]
struct RenderDyn {
    scope_ident: Ident,
    _comma_token: Token![,],
    _brace_token: syn::token::Brace,
    children: Children,
}

impl Parse for RenderDyn {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            scope_ident: input.parse()?,
            _comma_token: input.parse()?,
            _brace_token: braced!(content in input),
            children: content.parse()?,
        })
    }
}

/// Render a dynamic component.
#[proc_macro]
pub fn render_dyn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let render_dyn = parse_macro_input!(input as RenderDyn);

    generate_children_to_view(
        render_dyn.scope_ident,
        render_dyn.children.0.into_iter().collect(),
    )
    .into()
}

/// Implement `Render` from `RenderMulti`.
#[proc_macro_attribute]
pub fn render_from_multi(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let east_crate = east_crate();
    let input = parse_macro_input!(input as ItemImpl);
    let original = input.clone();

    let generics = input.generics;
    let last_trait_seg = input
        .trait_
        .expect("trait must exist")
        .1
        .segments
        .last()
        .expect("component type must exist")
        .clone();
    let last_trait_seg_args = last_trait_seg.arguments;
    assert_eq!(format!("{}", last_trait_seg.ident), "RenderMulti");

    let self_ty = input.self_ty;

    quote! {
        #original

        impl #generics #east_crate::Render #last_trait_seg_args for #self_ty {
            fn render(self) -> #east_crate::Markup {
                #east_crate::RenderMulti::#last_trait_seg_args::render_multi(self, Default::default())
            }
        }
    }.into()
}

/// Implement `Render` from `RenderDyn`.
#[proc_macro_attribute]
pub fn render_from_dyn(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let east_crate = east_crate();
    let input = parse_macro_input!(input as ItemImpl);
    let original = input.clone();

    let self_ty = input.self_ty;

    quote! {
        impl<AnyComponent> #east_crate::Render<AnyComponent> for #self_ty where
            AnyComponent: #east_crate::serde::Serialize + From<#self_ty>
        {
            fn render(self) -> #east_crate::Markup {
                let any_component = AnyComponent::from(self.clone());

                if let Ok(serialized) = #east_crate::json_to_string(&any_component) {
                    #east_crate::render_with_component!(AnyComponent, {
                        div {
                            data_component: serialized,
                            #east_crate::PreEscaped(#east_crate::render_to_string(|cx| {
                                self.render_dyn(cx)
                            })),
                        }
                    })
                } else {
                    #east_crate::render_with_component!(AnyComponent, {
                        div {
                            #east_crate::PreEscaped(#east_crate::render_to_string(|cx| {
                                self.render_dyn(cx)
                            })),
                        }
                    })
                }
            }
        }

        #original
    }
    .into()
}
