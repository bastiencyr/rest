use actix_web::{App, HttpServer};
use std::os::unix::net::UnixListener;
use std::os::unix::prelude::FromRawFd;

mod country;

use clap::Arg;
use listenfd::ListenFd;

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
                .long("systemd_unix")
                .help(
                    "Get a unix socket only from systemd thanks to sd_listen_fds(). \
                    You must configure \
                a service and a socket in systemd. Please refer to the example in \
                deployment/systemd_unix.",
                )
                .takes_value(false),
        )
        .arg(
            Arg::new("systemd_tcp")
                .long("systemd_tcp")
                .help(
                    "Get a tcp socket only from systemd thanks to sd_listen_fds(). \
                    You must configure \
                a service and a socket in systemd. Please refer to the example in \
                deployment/systemd_tcp.",
                )
                .takes_value(false),
        )
        .get_matches();

    let server = HttpServer::new(|| App::new().configure(country::init_routes)).workers(4);

    if matches.is_present("systemd_unix") {
        log::info!("Systemd unix socket option.");
        // in systemd, the socket use the first available file descriptor -> 3
        let lst = get_unix_listener();
        return server.listen_uds(lst)?.run().await;

        // another method is to use listenfd (see bellow)
    }

    if matches.is_present("systemd_tcp") {
        log::info!("Systemd TCP socket option.");
        let mut listenfd = ListenFd::from_env();
        let lst = listenfd.take_tcp_listener(0).unwrap().unwrap();
        return server.listen(lst)?.run().await;
    }

    // Finish by socket argument since socket arg has a default value
    if let Some(c) = matches.value_of("address") {
        log::info!("Standalone TCP socket option.");
        let sock: Vec<&str> = c.split(":").collect();
        return if sock.len() == 2 {
            server.bind(c)?.run().await
        } else {
            log::error!(
                "Fail to parse given URL. Unix socket URL are not supported : you can use \
            systemd_unix as a workaround."
            );
            Result::Ok(())
        };
    }

    return server.bind("127.0.0.1:8000")?.run().await;
}
