mod markup;
mod render;

pub use crate::markup::{PreEscaped, Markup, escape, escape_to_string};
pub use crate::render::{Render, RenderMulti, RenderDyn, NoComponent};
pub use east_macro::{render, render_with_component, render_from_multi, render_from_dyn};
pub use sycamore::prelude::*;
pub use sycamore::render_to_string;
pub use serde_json::to_string as json_to_string;

use serde::{Serialize, Deserialize};
