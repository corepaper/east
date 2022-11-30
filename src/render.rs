use crate::{builder, GenericNode, Markup, PreEscaped, Scope, View};
use serde::{Deserialize, Serialize};

/// Render the current value into a static markup.
///
/// In the case of dynamic components implementing `Render`,
/// server-side rendering (SSR) will be used. The data will be stored
/// in the `data-component` attribute of the root `div`, so that the
/// struct can be built and re-rendered on the client.
pub trait Render<Component> {
    fn render(self) -> Markup;
}

/// Render the current value, along with children, into a static
/// markup. This is the same as `Render` but with children.
///
/// After implementing `RenderMulti`, you can use the
/// `render_from_multi` macro to implement `Render`.
pub trait RenderMulti<Component> {
    fn render_multi(self, children: Markup) -> Markup;
}

/// Render a dynamic component.
///
/// After implementing `RenderDyn`, you can use `render_from_dyn`
/// macro to implement `Render`.
pub trait RenderDyn<G: GenericNode> {
    fn render_dyn(self, cx: Scope) -> View<G>;
}

impl<AnyComponent> Render<AnyComponent> for String {
    fn render(self) -> Markup {
        PreEscaped(crate::escape(&self))
    }
}

impl<'a, AnyComponent> Render<AnyComponent> for &'a str {
    fn render(self) -> Markup {
        PreEscaped(crate::escape(self))
    }
}

impl<T: AsRef<str>, AnyComponent> Render<AnyComponent> for PreEscaped<T> {
    fn render(self) -> Markup {
        let mut s = String::new();
        s.push_str(self.0.as_ref());
        PreEscaped(s)
    }
}

impl<G: GenericNode> RenderDyn<G> for String {
    fn render_dyn(self, _cx: Scope) -> View<G> {
        builder::t(self)
    }
}

impl<G: GenericNode> RenderDyn<G> for &'static str {
    fn render_dyn(self, _cx: Scope) -> View<G> {
        builder::t(self)
    }
}

/// Representing a renderer without any dynamic component.
#[derive(Serialize, Deserialize)]
pub enum NoComponent {}

impl<G: GenericNode> RenderDyn<G> for NoComponent {
    fn render_dyn(self, _cx: Scope) -> View<G> {
        match self {}
    }
}
