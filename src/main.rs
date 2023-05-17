use actix_web::{web, App, HttpServer, HttpResponse, Responder};
// use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
// use dotenv::dotenv;

// mod services;
// use services::{fetch_data};
async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("front/index.html"))
}
use actix_files as fs;
// pub struct AppState {
//     db: Pool<Postgres>
// }
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().ok();
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let pool = PgPoolOptions::new()
    // .max_connections(5)
    // .connect(&database_url)
    // .await
    // .expect("Error building a connection pool");

    HttpServer::new(|| 
        App::new()
            .route("/", web::get().to(index))
            .service(fs::Files::new("/static", "./").show_files_listing())
            // .app_data(Data::new(AppState { db: pool.clone() }))
            // .service(fetch_data)
    )
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
