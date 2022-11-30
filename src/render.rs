use serde::{Serialize, Deserialize};
use crate::{Markup, PreEscaped, GenericNode, Scope, View, builder};

pub trait Render<Component> {
    fn render(self) -> Markup;
}

pub trait RenderMulti<Component> {
    fn render_multi(self, children: Markup) -> Markup;
}

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

#[derive(Serialize, Deserialize)]
pub enum NoComponent { }

impl<G: GenericNode> RenderDyn<G> for NoComponent {
    fn render_dyn(self, cx: Scope) -> View<G> {
        match self { }
    }
}
