use east::{Render, RenderDyn, Markup, GenericNode, render_with_component, render_from_dyn, render_dyn};
use east::sycamore::prelude::*;
use web_sys::Node;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AnyComponent {
    Counter(Counter),
}

impl From<Counter> for AnyComponent {
    fn from(counter: Counter) -> Self {
        Self::Counter(counter)
    }
}

impl AnyComponent {
    pub fn hydrate_to(self, parent: &Node) {
        match self {
            Self::Counter(counter) => {
                east::sycamore::hydrate_to(
                    move |cx| counter.render_dyn(cx),
                    parent,
                );
            }
        }
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

impl<AnyComponent> Render<AnyComponent> for Index where
    AnyComponent: Serialize + From<Counter>
{
    fn render(self) -> Markup {
        render_with_component!(AnyComponent, {
            "This is static",
            Counter { id: 1 },
            Counter { id: 2 },
        })
    }
}
