use east::{Partial, NoComponent, view};

pub struct Index;

impl<AnyComponent> Partial<AnyComponent> for Index {
    fn view(&self) -> String {
        view! {
            div {
                class: "test-class",

                "This is a test page.",
                button { "Click me!" }
            }
        }
    }
}

#[test]
fn test_basic_macro() {
    assert_eq!(Partial::<NoComponent>::view(&Index), "<div class=\"test-class\">This is a test page.</div>");
}
