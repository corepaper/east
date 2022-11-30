use east_example_counter_ui::AnyComponent;
use wasm_bindgen::JsCast;

fn main() {
    web_sys::console::log_1(&"Counter".into());

    let dyns = web_sys::window().expect("window exist")
        .document().expect("document exist")
        .query_selector_all("[data-component]").expect("query dyns succeed");

    for i in 0..dyns.length() {
        let item = dyns.get(i).expect("index within length").dyn_into::<web_sys::Element>().expect("dyn node is an element");
        let any_component: AnyComponent = serde_json::from_str(&item.get_attribute("data-component").expect("data-component attribute exist")).expect("parse component json succeed");

        any_component.hydrate_to(&item);

        let hk_nodes = item.query_selector_all("[data-hk]").expect("query sub nodes succeed");
        for i in 0..hk_nodes.length() {
            let item = hk_nodes.get(i).expect("index within length").dyn_into::<web_sys::Element>().expect("hk node is an element");
            item.remove_attribute("data-hk").expect("removing attribute succeed");
        }
    }
}
