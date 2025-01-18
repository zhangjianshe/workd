mod api;
mod dir;
mod page;
mod router;

use crate::router::create_route;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!(
        "workerd is listening on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, create_route()).await.unwrap();
}
