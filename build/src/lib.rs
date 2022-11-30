//! Build integration for East.

pub use axum;
pub use east;
pub use include_dir;
pub use mime;

#[macro_export]
macro_rules! include_trunk_assets {
    {
        Asset = $asset_ty:ident,
        Html = $html_ty:ident,
        TRUNK_ASSET_FILES = $asset_files:ident,
        route_trunk_assets = $route_assets:ident,
        replace_header = $replace_header:expr,
        replace_body = $replace_body:expr,
    } => {
        static $asset_files: $crate::include_dir::Dir<'static> = $crate::include_dir::include_dir!("$OUT_DIR/dist");

        /// Asset type.
        pub struct $asset_ty {
            pub ext: Option<String>,
            pub content: Vec<u8>,
        }

        impl $crate::axum::response::IntoResponse for $asset_ty {
            fn into_response(self) -> $crate::axum::response::Response {
                use std::str::FromStr;

                let mime = match self.ext.as_ref().map(|s| s.as_ref()) {
                    Some("html") => Some($crate::mime::TEXT_HTML),
                    Some("css") => Some($crate::mime::TEXT_CSS),
                    Some("js") => Some($crate::mime::TEXT_JAVASCRIPT),
                    Some("wasm") => {
                        Some($crate::mime::Mime::from_str("application/wasm").expect("wasm is valid mime"))
                    }
                    _ => None,
                };

                if let Some(mime) = mime {
                    ($crate::axum::TypedHeader($crate::axum::headers::ContentType::from(mime)), self.content).into_response()
                } else {
                    self.content.into_response()
                }
            }
        }

        /// Route assets.
        pub fn $route_assets(mut app: $crate::axum::Router) -> $crate::axum::Router {
            for entry in $asset_files.entries() {
                if let Some(file) = entry.as_file() {
                    let ext = file
                        .path()
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_owned());
                    let path = file.path().to_string_lossy();

                    if path != "index.html" {
                        let content = file.contents().to_vec();

                        app = app.route(
                            &("/".to_string() + &path),
                            $crate::axum::routing::get(move || async { $asset_ty { ext, content } }),
                        );
                    }
                }
            }

            app
        }

        /// Html type.
        #[derive(Debug, Clone)]
        pub struct $html_ty {
            pub header: $crate::east::Markup,
            pub body: $crate::east::Markup,
        }

        impl $crate::axum::response::IntoResponse for $html_ty {
            fn into_response(self) -> $crate::axum::response::Response {
                let template_file = $asset_files.get_file("index.html").expect("index file exist");
                let template = template_file.contents_utf8().expect("template is valid utf8");

                $crate::axum::response::IntoResponse::into_response($crate::axum::response::Html(
                    template
                        .replace($replace_header, &self.header.0)
                        .replace($replace_body, &self.body.0)
                ))
            }
        }
    }
}
