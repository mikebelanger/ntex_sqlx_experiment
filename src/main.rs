use ntex::web;
use ntex_files as fs;
pub mod view;

use view::*;

use sqlx::{postgres::PgPoolOptions, Error, PgPool, pool::PoolConnectionMetadata, Postgres};
use std::env;

async fn get_pool() -> Result<PgPool, Error> {
    // Get database address
    let db_address = env::var("DATABASE_URL");

    match db_address {
        Ok(addr) => {
            return PgPoolOptions::new().max_connections(5).connect(&addr).await;
        }
        Err(_) => panic!("DATABASE_URL not set. Please define this env variable."),
    }
}

#[web::get("/")]
async fn index(
    pool: web::types::State<PgPool>,
) -> impl web::Responder {
    let todos = sqlx::query_as::<_, Todo>("SELECT title, content FROM todo")
        .fetch_all(&*pool)
        .await
        .unwrap();

    web::HttpResponse::Ok().body(
        MainPage {
            title: "Welcome",
            name: "Mike",
            todos: &todos,
        }
        .to_string(),
    )
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let pool = get_pool().await.expect("error starting connection pool");
    web::HttpServer::new(move || {
        web::App::new()
            .state(pool.clone())
            .service(index)
            .service(fs::Files::new("/", "./static_assets/"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
