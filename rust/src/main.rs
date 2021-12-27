use actix_web::{App, HttpServer};
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().configure(user::init_routes));
    server.bind("127.0.0.1:5000")?.run().await
}
