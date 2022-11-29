mod markup;
mod render;

pub use crate::markup::{PreEscaped, Markup, escape, escape_to_string};
pub use crate::render::{Render, RenderMulti, RenderDyn, NoComponent};
pub use east_macro::{view, view_with_component, render_from_multi};
pub use sycamore::prelude::*;

use serde::{Serialize, Deserialize};



