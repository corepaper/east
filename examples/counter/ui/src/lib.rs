use east::{DynViewBuilder, Component, Partial, Markup, view_with_component};
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

impl Component for Counter {
    fn view_builder<G: GenericNode>(&self) -> DynViewBuilder<G> {
        let id = self.id;

        DynViewBuilder(Box::new(move |cx| {
            let id = create_signal(cx, id);

            view!(cx, button(on:click = |_| id.set(2)) { "Click me" (*id.get()) })
        }))
    }
}

pub struct Index;

impl<AnyComponent> Partial<AnyComponent> for Index where
    AnyComponent: From<Counter>
{
    fn view(&self) -> Markup {
        view_with_component!(AnyComponent, {
            Counter { id: 1 },
            Counter { id: 2 },
        })
    }
}
