use syn::{Result, Ident, Token, parenthesized, bracketed, braced, parse::{Parse, ParseStream}, ext::IdentExt, token::{Paren, Bracket}, punctuated::Punctuated};

pub static HTML_TAGS: [&'static str; 78] = [
    "a", "abbr", "acronym", "address", "area",
    "b", "base", "bdo", "big", "blockquote", "body", "br", "button",
    "caption", "cite", "code", "col", "colgroup",
    "dd", "del", "dfn", "div", "dl", "DOCTYPE", "dt",
    "em",
    "fieldset", "form",
    "h1", "h2", "h3", "h4", "h5", "h6", "head", "html", "hr",
    "i", "img", "input", "ins",
    "kbd",
    "label", "legend", "li", "link",
    "map", "meta",
    "noscript",
    "object", "ol", "optgroup", "option",
    "p", "param", "pre",
    "q",
    "samp", "script", "select", "small", "span", "strong", "style", "sub", "sup",
    "table", "tbody", "td", "textarea", "tfoot", "th", "thead", "title", "tr", "tt",
    "ul",
    "var"
];

fn is_element(input: ParseStream) -> bool {
    let input = input.fork();
    input.parse::<syn::ExprPath>().is_ok() && input.peek(syn::token::Brace)
}

#[derive(Debug, Clone)]
pub enum Child {
    Element(Element),
    Expr(syn::Expr),
}

impl Parse for Child {
    fn parse(input: ParseStream) -> Result<Self> {
        if is_element(input) {
            Ok(Child::Element(input.parse()?))
        } else {
            Ok(Child::Expr(input.parse()?))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Children(pub Punctuated<Child, Token![,]>);

impl Parse for Children {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Children(input.parse_terminated(Child::parse)?))
    }
}

#[derive(Debug, Clone)]
pub enum AttributeOrChild {
    Attribute(Attribute),
    Child(Child),
}

impl Parse for AttributeOrChild {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::Ident) && input.peek2(Token![:]) && !input.peek3(Token![:]) {
            Ok(Self::Attribute(input.parse()?))
        } else {
            Ok(Self::Child(input.parse()?))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub tag: syn::Path,
    pub brace_token: syn::token::Brace,
    pub attributes_or_children: Punctuated<AttributeOrChild, Token![,]>,
}

impl Element {
    pub fn html_tag(&self) -> Option<String> {
        self.tag.get_ident().map(|i| i.to_string())
            .and_then(|i| if HTML_TAGS.contains(&i.as_str()) { Some(i) } else { None })
    }

    pub fn attributes(&self) -> Vec<Attribute> {
        let mut attributes = Vec::new();

        for attribute_or_child in &self.attributes_or_children {
            if let AttributeOrChild::Attribute(attribute) = &attribute_or_child {
                attributes.push(attribute.clone());
            }
        }

        attributes
    }

    pub fn children(&self) -> Vec<Child> {
        let mut children = Vec::new();

        for attribute_or_child in &self.attributes_or_children {
            if let AttributeOrChild::Child(child) = &attribute_or_child {
                children.push(child.clone());
            }
        }

        children
    }
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            tag: input.parse()?,
            brace_token: braced!(content in input),
            attributes_or_children: content.parse_terminated(AttributeOrChild::parse)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: syn::Ident,
    pub colon_token: Token![:],
    pub value: syn::Expr,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            name: input.parse()?,
            colon_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
