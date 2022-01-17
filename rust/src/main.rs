use actix_web::{App, HttpServer};
use std::os::unix::net::UnixListener;
use std::os::unix::prelude::FromRawFd;

mod country;

use clap::Arg;

fn get_unix_listener() -> UnixListener {
    return unsafe { UnixListener::from_raw_fd(3) };
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let matches = clap::App::new("Rest api")
        .version("1.0")
        .author("Bastien")
        .about("Simple rest API in Rust")
        .arg(
            Arg::new("socket")
                .long("socket")
                .short('s')
                .help(
                    "The socket to run the server. For TCP socket: 127.0.0.1:5000. \
        Unix socket are not supported",
                )
                .default_value("127.0.0.1:5000")
                .value_name("SOCKET"),
        )
        .arg(
            Arg::new("systemd_unix")
                .help("Get socket from systemd thanks to sd_listen_fds()")
                .takes_value(false),
        )
        .get_matches();

    let server = HttpServer::new(|| App::new().configure(country::init_routes)).workers(4);

    if let Some(_) = matches.value_of("systemd_unix") {
        // in systemd, the socket use the first available file descriptor -> 3
        let lst = get_unix_listener();
        return server.listen_uds(lst)?.run().await;

        // another method with listenfd
        //use listenfd::ListenFd;
        //let mut listenfd = ListenFd::from_env();
        //let lst = listenfd.take_unix_listener(0).unwrap().unwrap();
    }

    // Finish by socket argument since socket arg has a default value
    if let Some(c) = matches.value_of("socket") {
        let sock: Vec<&str> = c.split(":").collect();
        if sock.len() == 2 {
            return server.bind(c)?.run().await;
        } else {
            panic!("Fail to parse given URL. Unix socket URL are not supported : you can use systemd_unix as a workaround.")
        }
    }

    return server.bind("127.0.0.1:8000")?.run().await;
}
