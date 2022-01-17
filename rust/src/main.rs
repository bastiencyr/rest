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
            Arg::new("address")
                .long("addr")
                .short('s')
                .help(
                    "The socket to run the server. For TCP socket: 127.0.0.1:5000.\
                    Unix socket are not supported.",
                )
                .default_value("127.0.0.1:5000")
                .value_name("SOCKET"),
        )
        .arg(
            Arg::new("systemd_unix")
                .help(
                    "Get socket from systemd thanks to sd_listen_fds(). You must configure \
                a service and a socket in systemd. Please refer to the example in \
                deployment/systemd.",
                )
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
    if let Some(c) = matches.value_of("address") {
        let sock: Vec<&str> = c.split(":").collect();
        return if sock.len() == 2 {
            server.bind(c)?.run().await
        } else {
            log::error!("Fail to parse given URL. Unix socket URL are not supported : you can use systemd_unix as a workaround.");
            Result::Ok(())
        };
    }

    return server.bind("127.0.0.1:8000")?.run().await;
}
