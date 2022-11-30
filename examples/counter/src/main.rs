use axum::{
    headers::ContentType,
    response::{Html, IntoResponse, Response},
    routing,
    routing::get,
    Router, TypedHeader,
};
use east::{render, render_with_component, Markup, Render};
use std::{net::SocketAddr, str::FromStr};
use ui::{AnyComponent, Index};

static UI_FILES: include_dir::Dir<'static> = include_dir::include_dir!("$OUT_DIR/dist");

pub struct Asset {
    ext: Option<String>,
    content: Vec<u8>,
}

impl IntoResponse for Asset {
    fn into_response(self) -> Response {
        let mime = match self.ext.as_ref().map(|s| s.as_ref()) {
            Some("html") => Some(mime::TEXT_HTML),
            Some("css") => Some(mime::TEXT_CSS),
            Some("js") => Some(mime::TEXT_JAVASCRIPT),
            Some("wasm") => {
                Some(mime::Mime::from_str("application/wasm").expect("wasm is valid mime"))
            }
            _ => None,
        };

        if let Some(mime) = mime {
            (TypedHeader(ContentType::from(mime)), self.content).into_response()
        } else {
            self.content.into_response()
        }
    }
}

fn html(header: Markup, body: Markup) -> Html<String> {
    let template_file = UI_FILES.get_file("index.html").expect("index file exist");
    let template = template_file
        .contents_utf8()
        .expect("template is valid utf8");

    Html(
        template
            .replace("<!-- header -->", &header.0)
            .replace("<!-- body -->", &body.0),
    )
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let mut app = Router::new();

    for entry in UI_FILES.entries() {
        if let Some(file) = entry.as_file() {
            let ext = file
                .path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_owned());
            let path = file.path().to_string_lossy();
            let content = file.contents().to_vec();

            app = app.route(
                &("/".to_string() + &path),
                routing::get(move || async { Asset { ext, content } }),
            );
        }
    }

    app = app.route("/", routing::get(index));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<String> {
    east::sycamore::utils::hydrate::with_hydration_context(|| {
        html(
            render! {
                title { "Counter" }
            },
            render_with_component!(AnyComponent, { Index {} }),
        )
    })
}
