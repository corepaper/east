//! East is an opinioned Rust full-stack web library for island
//! architecture.

mod markup;
mod render;
mod hydrate;

pub use crate::markup::{escape, escape_to_string, Markup, PreEscaped};
pub use crate::render::{NoComponent, Render, RenderDyn, RenderMulti};
pub use crate::hydrate::{HydrateTo, hydrate_all};
pub use east_macro::{
    render, render_dyn, render_from_dyn, render_from_multi, render_with_component, HydrateTo,
};
pub use serde;
pub use serde_json::to_string as json_to_string;
pub use sycamore;
pub use sycamore::prelude::*;
pub use sycamore::{builder, render_to_string};
pub use web_sys;
