use east::Render;
use ui::{AnyComponent, Index};

fn main() {
    println!("{:?}", Render::<AnyComponent>::render(Index));
}
