use east::Partial;
use ui::{AnyComponent, Index};

fn main() {
    println!("{}", Partial::<AnyComponent>::view(&Index));
}
