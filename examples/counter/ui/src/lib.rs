use east::sycamore::prelude::*;
use east::{
    HydrateTo,
    render_dyn, render_from_dyn, render_with_component, GenericNode, Markup, Render, RenderDyn,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, HydrateTo, Debug, Clone)]
pub enum AnyComponent {
    Counter(Counter),
}

impl From<Counter> for AnyComponent {
    fn from(counter: Counter) -> Self {
        Self::Counter(counter)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Counter {
    pub id: usize,
}

#[render_from_dyn]
impl<G: GenericNode> RenderDyn<G> for Counter {
    fn render_dyn(self, cx: Scope) -> View<G> {
        let value = create_signal(cx, 0);

        render_dyn!(cx, {
            button {
                on_click: |_| value.set(*value.get() + 1),

                "Click me ", self.id.to_string(),
            },
            p {
                "value: ", value.get().to_string(),
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Index;

impl<AnyComponent> Render<AnyComponent> for Index
where
    AnyComponent: Serialize + From<Counter>,
{
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            "This is static",
            Counter { id: 1 },
            Counter { id: 2 },
        })
    }
}
