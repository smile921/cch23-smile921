use axum::routing::get;
use tower_http::services::ServeDir;
use tower_http::trace;

mod days;
mod test;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

struct CCHService {
    router: axum::Router,
}

#[allow(clippy::unused_async)]
#[shuttle_runtime::main]
async fn init() -> Result<CCHService, shuttle_runtime::Error> {
    let router_days = days::router();
    let router = router_days
        .route("/hello", get(hello_world))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        );

    Ok(CCHService { router })
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CCHService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = self.router;
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        tokio::select!(() = async {axum::serve(listener, router).await.unwrap()} => {});

        Ok(())
    }
}

// let router = Router::new()
//     .route("/", get(hello_world))
//     .nest_service("/assets", ServeDir::new("assets"));
