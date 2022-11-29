use east::{Render, RenderDyn, Markup, render_with_component, render_from_dyn, render_dyn};
use sycamore::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum AnyComponent {
    Counter(Counter),
}

impl From<Counter> for AnyComponent {
    fn from(counter: Counter) -> Self {
        Self::Counter(counter)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Counter {
    pub id: usize,
}

#[render_from_dyn]
impl<G: GenericNode> RenderDyn<G> for Counter {
    fn render_dyn(self, cx: Scope) -> View<G> {
        let id = create_signal(cx, self.id);

        render_dyn!(cx, {
            button {
                on_click: |_| id.set(2),

                "Click me", id.get().to_string()
            }
        })
    }
}

pub struct Index;

impl<AnyComponent> Render<AnyComponent> for Index where
    AnyComponent: From<Counter>
{
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            Counter { id: 1 },
            Counter { id: 2 },
        })
    }
}
