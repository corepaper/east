use crate::{Markup, PreEscaped, GenericNode, Scope, View};

pub trait Render<Component> {
    fn render(self) -> Markup;
}

pub trait RenderMulti<Component> {
    fn render_multi(self, children: Markup) -> Markup;
}

pub trait RenderDyn<G: GenericNode> {
    fn render_dyn(self, cx: Scope) -> View<G>;
}

// impl<T: DynRender + Serialize, AnyComponent> Render<AnyComponent> for T where
//     AnyComponent: From<T>
// {
//     fn render(self) -> Markup {
//         if let Ok(serialized) = serde_json::to_string(&self) {
//             render_with_component!(AnyComponent, {
//                 div {
//                     data_component: serialized,
//                     PreEscaped(sycamore::render_to_string(|cx| {
//                         self.dyn_render(cx)
//                     })),
//                 }
//             })
//         } else {
//             render_with_component!(AnyComponent, {
//                 div {
//                     PreEscaped(sycamore::render_to_string(|cx| {
//                         self.dyn_render(cx)
//                     })),
//                 }
//             })
//         }
//     }
// }

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

pub enum NoComponent { }

impl<G: GenericNode> RenderDyn<G> for NoComponent {
    fn render_dyn(self, cx: Scope) -> View<G> {
        match self { }
    }
}
