use east::{Partial, NoComponent, Markup, view};

#[derive(Default)]
pub struct Index;

impl<AnyComponent> Partial<AnyComponent> for Index {
    fn view_multi(&self, children: Markup) -> Markup {
        view! {
            div {
                class: "test-class",

                "This is a test page.",
                children,
            }
        }
    }
}

#[test]
fn test_basic_macro() {
    let view = view! {
        Index {
            button { "Click me!" }
        }
    };
    assert_eq!(view.0, "<div class=\"test-class\">This is a test page.<button>Click me!</button></div>");
}
