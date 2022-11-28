mod node;

use syn::{Result, Ident, Token, parse_macro_input, parenthesized, bracketed, braced, parse::{Parse, ParseStream}, ext::IdentExt, token::{Paren, Bracket}};
use proc_macro2::Span;
use quote::quote;
use crate::node::{Child, Children};

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
    let output = Ident::new("__east_output", Span::mixed_site());

    let children = children.into_iter().map(|child| {
        match child {
            Child::Element(element) => {
                if let Some(html_tag) = element.html_tag() {
                    let attributes = element.attributes().into_iter().map(|attribute| {
                        let name = proc_macro2::Literal::string(&format!("{}", attribute.name));
                        let value = attribute.value;
                        quote! {
                            format!("{}=\"{}\"", #name, ::east::escape::escape(#value))
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
                            #output.push_str(&::east::Partial::<#component_type>::view(&#tag {
                                #(#attributes),*
                                ..Default::default()
                            }).0);
                        }
                    } else {
                        let children = generate_children_to_string(component_type.clone(), element.children());

                        quote! {
                            #output.push_str(&::east::Partial::<#component_type>::view(&#tag {
                                #(#attributes),*
                                children: #children,
                                ..Default::default()
                            }).0);
                        }
                    }
                }
            },
            Child::Expr(expr) => {
                quote! { #output.push_str(&::east::Partial::<#component_type>::view(&#expr).0); }
            },
        }
    });

    quote! { {
        let mut #output = String::new();

        #(#children)*

        ::east::PreEscaped(#output)
    } }
}

#[proc_macro]
pub fn view(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let view = parse_macro_input!(input as View);
    let component_type = quote! { ::east::NoComponent };

    generate_children_to_string(component_type, view.children.0.into_iter().collect()).into()
}
