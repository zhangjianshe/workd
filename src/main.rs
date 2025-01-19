mod api;
mod dir;
mod page;
mod router;

use crate::router::create_route;
use satway_build::CompileInfo;
#[tokio::main]
async fn main() {

let info=include_str!("context/info.txt");
    let info= CompileInfo::load_from_str(info);
    match info { 
        Ok(_) => println!("Compile info loaded successfully"),
        Err(e) => eprintln!("Failed to load compile info: {}", e),
    }
    let listener = tokio::net::TcpListener::bind("127.0.0.1:1234")
        .await
        .unwrap();
    println!(
        "workerd is listening on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, create_route()).await.unwrap();
}


