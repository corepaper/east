use east::{render, render_from_multi, Markup, NoComponent, Render, RenderMulti};

#[derive(Default)]
pub struct Index;

#[render_from_multi]
impl<AnyComponent> RenderMulti<AnyComponent> for Index {
    fn render_multi(self, children: Markup) -> Markup {
        render! {
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
    let view = render! {
        Index {
            button { "Click me!" }
        }
    };
    assert_eq!(
        view.0,
        "<div class=\"test-class\">This is a test page.<button>Click me!</button></div>"
    );
}
