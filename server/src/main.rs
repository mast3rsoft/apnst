use axum::{
    routing::get,
    Router,
};
use tracing::{info,error, Level};
use tracing_subscriber::FmtSubscriber;
use tokio::main;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
    // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
    // will be written to stdout.
    .with_max_level(Level::TRACE)
    // completes the builder.
    .finish();

tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Starting server at 0.0.0.0:3000");
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    info!("Connecting to DB");
    if let Err(nig) =   PgPoolOptions::new().max_connections(5).connect("postgres://postgres:12345678@localhost/postgres").await
     {
        error!("Error while connecting");
        std::process::exit(1);
    }
    info!("DB Connected!");

    axum::serve(listener, app).await.unwrap();
    
}