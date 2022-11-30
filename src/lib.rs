//! East is an opinioned Rust full-stack web library for island
//! architecture.

mod hydrate;
mod markup;
mod render;

pub use crate::hydrate::{hydrate_all, HydrateTo};
pub use crate::markup::{escape, escape_to_string, Markup, PreEscaped};
pub use crate::render::{NoComponent, Render, RenderDyn, RenderMulti};
pub use east_macro::{
    render, render_dyn, render_from_dyn, render_from_multi, render_with_component, HydrateTo,
};
pub use serde;
pub use serde_json::to_string as json_to_string;
pub use sycamore;
pub use sycamore::prelude::*;
pub use sycamore::{builder, render_to_string};
pub use web_sys;
