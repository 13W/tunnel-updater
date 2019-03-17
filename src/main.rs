use actix_web::{server, http, middleware, App, HttpRequest, HttpResponse, http::Method, Responder};

use std::env::{var, set_var};
use std::sync::Arc;
use std::process::Command;

mod config;

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
    if var("RUST_LOG").is_err() {
        set_var("RUST_LOG", "actix_web=debug");
    }

    env_logger::init();

    let config = config::get();
    let sys = actix::System::new("static_index");
    let secret = Arc::new(config.secret);

    let mut app = server::new(move || {
        App::with_state(AppState{secret: secret.clone()})
            .middleware(middleware::Logger::default())
            .route("/update", Method::GET, index)
    });

    app = app.bind(&format!("{}:{}", &config.host, &config.port))
        .expect(format!("Can not start on {}:{}", &config.host, &config.port).as_str());

    app.start();

    println!("Tunnel-updater started on {}:{}", &config.host, &config.port);
    sys.run();
}
