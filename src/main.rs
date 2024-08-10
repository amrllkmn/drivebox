mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let app = routes::create_api_route();
    let addr = "0.0.0.0:8080"; // todo: change me to run without docker
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
