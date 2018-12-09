extern crate env_logger;
extern crate actix;
extern crate actix_web;
extern crate clap;

use actix_web::{server, http, middleware, App, HttpRequest, HttpResponse, http::Method, Responder};

use std::sync::Arc;
use std::process::Command;
use clap::*;

struct AppState {
    secret: Arc<String>,
}

fn index(req: HttpRequest<AppState>) -> impl Responder {
    let info = req.connection_info();
    let query = req.query();
    let state: &AppState = req.state();

    let x_forwarded_for = info.remote().unwrap();
    let tunnel = query.get("tunnel");
    let secret = query.get("key");

    if tunnel.is_none() || secret.is_none() {
        return HttpResponse::with_body(http::StatusCode::BAD_REQUEST, "Bad request.\n");
    }

    if *secret.unwrap() != *state.secret {
        return HttpResponse::with_body(http::StatusCode::UNAUTHORIZED, "Unauthorized.\n");
    }

    let tunnel = tunnel.unwrap();
    let remote_ip = &req.peer_addr().unwrap().ip().to_string();

    let result = Command::new("/sbin/ip")
        .arg("tunnel")
        .arg("change")
        .arg(tunnel)
        .arg("remote")
        .arg(remote_ip)
        .output()
        .expect("Filed to execute command.");

    let result = format!("\
        Peer address: {}\n\
        X-Forwarded-For: {}\n\
        Tunnel: {}\n\
        Result: {}\n\
    ", remote_ip, x_forwarded_for, tunnel, String::from_utf8(result.stderr).expect("Failed to update."));

    HttpResponse::from(result)
}
fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let matches = clap::App::new("IP tunnel updater")
        .version(crate_version!())
        .about("Tools for updating the remote tunnel address")

        .arg(clap::Arg::with_name("address")
            .help("host address")
            .short("a")
            .env("ADDRESS")
            .default_value("0.0.0.0"))

        .arg(clap::Arg::with_name("port")
            .help("host port")
            .short("p")
            .env("PORT")
            .default_value("38123"))

        .arg(clap::Arg::with_name("secret")
            .help("update key")
            .short("s")
            .env("SECRET")
            .default_value("SeCrEtKeY"))
        .get_matches();

    let host = matches.value_of("address").unwrap();
    let port = matches.value_of("port").unwrap()
        .parse::<u32>().expect("invalid port number");
    let secret = Arc::new(matches.value_of("secret").unwrap().to_string());

    let sys = actix::System::new("static_index");

    server::new(move || {
        App::with_state(AppState{secret: secret.clone()/*, log: logger.clone()*/})
            .middleware(middleware::Logger::default())
            .resource("/update", |r| r.method(Method::GET).with(index))

    }).bind(&format!("{}:{}", host, port))
        .expect(format!("Can not start on {}:{}", host, port).as_str())
        .start();

    println!("Tunnel-updater started on {}:{}", host, port);
    sys.run();
}
