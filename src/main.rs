mod handlers;
mod routes;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let addr = "0.0.0.0:8080"; // todo: change me to run without docker
    let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env");

    let app_state = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!()
        .run(&app_state)
        .await
        .expect("Migrating went wrong");
    let app = routes::create_api_route(app_state);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
