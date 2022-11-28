pub mod escape;

use serde::{Serialize, Deserialize};
use sycamore::{view::View, generic_node::GenericNode, reactive::Scope};

pub use east_macro::view;

pub struct DynViewBuilder<G: GenericNode>(pub Box<dyn FnOnce(Scope) -> View<G>>);

pub trait Component {
    fn view_builder<G: GenericNode>(&self) -> DynViewBuilder<G>;
}

pub trait Partial<Component> {
    fn view(&self) -> Markup;
}

impl<T: Component, AnyComponent> Partial<AnyComponent> for T where
    AnyComponent: From<T>,
{
    fn view(&self) -> Markup {
        PreEscaped(sycamore::render_to_string(|cx| {
            self.view_builder().0(cx)
        }))
    }
}

impl<AnyComponent> Partial<AnyComponent> for String {
    fn view(&self) -> Markup {
        PreEscaped(crate::escape::escape(self))
    }
}

impl<'a, AnyComponent> Partial<AnyComponent> for &'a str {
    fn view(&self) -> Markup {
        PreEscaped(crate::escape::escape(self))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PreEscaped<T: AsRef<str>>(pub T);

impl<T: AsRef<str>, AnyComponent> Partial<AnyComponent> for PreEscaped<T> {
    fn view(&self) -> Markup {
        let mut s = String::new();
        s.push_str(self.0.as_ref());
        PreEscaped(s)
    }
}

pub type Markup = PreEscaped<String>;

pub enum NoComponent { }

impl Component for NoComponent {
    fn view_builder<G: GenericNode>(&self) -> DynViewBuilder<G> {
        match *self { }
    }
}
