use actix_web::{App, HttpServer};
mod country;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init the server with routes
    let server = HttpServer::new(|| App::new().configure(country::init_routes));
    server.bind("127.0.0.1:5001")?.run().await
}
