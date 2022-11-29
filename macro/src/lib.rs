mod node;

use syn::{Result, Ident, Token, ItemImpl, parse_macro_input, parenthesized, bracketed, braced, parse::{Parse, ParseStream}, ext::IdentExt, token::{Paren, Bracket}};
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use crate::node::{Child, Children};

fn east_crate() -> proc_macro2::TokenStream {
    let found_crate = crate_name("east").expect("east is present in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => quote!( crate ),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident )
        }
    }
}

#[derive(Clone, Debug)]
struct View {
    children: Children,
}

impl Parse for View {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            children: input.parse()?,
        })
    }
}

#[derive(Clone, Debug)]
struct ViewWithComponent {
    component_type: syn::Type,
    comma_token: Token![,],
    brace_token: syn::token::Brace,
    children: Children,
}

impl Parse for ViewWithComponent {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            component_type: input.parse()?,
            comma_token: input.parse()?,
            brace_token: braced!(content in input),
            children: content.parse()?,
        })
    }
}

fn generate_children_to_string(component_type: proc_macro2::TokenStream, children: Vec<Child>) -> proc_macro2::TokenStream {
    let east_crate = east_crate();
    let output = Ident::new("__east_output", Span::mixed_site());

    let children = children.into_iter().map(|child| {
        match child {
            Child::Element(element) => {
                if let Some(html_tag) = element.html_tag() {
                    let attributes = element.attributes().into_iter().map(|attribute| {
                        let name = proc_macro2::Literal::string(&format!("{}", attribute.name));
                        let value = attribute.value;
                        quote! {
                            format!("{}=\"{}\"", #name, #east_crate::escape(#value))
                        }
                    });

                    if element.children().is_empty() {
                        quote! {
                            #output.push_str("<");
                            #output.push_str([#html_tag, #(&#attributes),*].join(" "));
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

#[proc_macro]
pub fn view(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let east_crate = east_crate();
    let view = parse_macro_input!(input as View);
    let component_type = quote! { #east_crate::NoComponent };

    generate_children_to_string(component_type, view.children.0.into_iter().collect()).into()
}

#[proc_macro]
pub fn view_with_component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let view_with_component = parse_macro_input!(input as ViewWithComponent);
    let component_type = view_with_component.component_type;

    generate_children_to_string(quote! { #component_type }, view_with_component.children.0.into_iter().collect()).into()
}

#[proc_macro_attribute]
pub fn render_from_multi(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let east_crate = east_crate();
    let input = parse_macro_input!(input as ItemImpl);
    let original = input.clone();

    let generics = input.generics;
    let last_trait_seg = input.trait_.expect("trait must exist").1
        .segments.last().expect("component type must exist").clone();
    let last_trait_seg_args = last_trait_seg.arguments;
    assert_eq!(format!("{}", last_trait_seg.ident), "RenderMulti");

    let self_ty = input.self_ty;

    quote! {
        impl #generics #east_crate::Render #last_trait_seg_args for #self_ty {
            fn render(self) -> #east_crate::Markup {
                #east_crate::RenderMulti::#last_trait_seg_args::render_multi(self, Default::default())
            }
        }

        #original
    }.into()
}
