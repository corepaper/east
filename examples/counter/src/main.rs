use axum::{routing, Router};
use east::{render, render_with_component};
use std::net::SocketAddr;
use ui::{AnyComponent, Index};

east_build::include_trunk_assets! {
    Asset = Asset,
    Html = Html,
    TRUNK_ASSET_FILES = TRUNK_ASSET_FILES,
    route_trunk_assets = route_trunk_assets,
    replace_header = "<!-- header -->",
    replace_body = "<!-- body -->",
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let mut app = Router::new();

    app = route_trunk_assets(app);

    app = app.route("/", routing::get(index));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html {
    Html {
        header: render! {
            title { "Counter" }
        },
        body: render_with_component!(AnyComponent, { Index {} }),
    }
}
