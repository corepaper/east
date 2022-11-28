use serde::{Serialize, Deserialize};
use sycamore::{view::View, generic_node::GenericNode, reactive::Scope};

pub use east_macro::view;

pub struct DynViewBuilder<G: GenericNode>(pub Box<dyn FnOnce(Scope) -> View<G>>);

pub trait Component {
    fn view_builder<G: GenericNode>(&self) -> DynViewBuilder<G>;
}

pub trait Partial<Component> {
    fn view(&self) -> String;
}

impl<T: Component, AnyComponent> Partial<AnyComponent> for T where
    AnyComponent: From<T>,
{
    fn view(&self) -> String {
        sycamore::render_to_string(|cx| {
            self.view_builder().0(cx)
        })
    }
}

pub enum NoComponent { }

impl Component for NoComponent {
    fn view_builder<G: GenericNode>(&self) -> DynViewBuilder<G> {
        match *self { }
    }
}
