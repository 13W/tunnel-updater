#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::env;
use std::process::Command;
use rocket::{Outcome, Request};
use rocket::request::{self, FromRequest};
use rocket::http::Status;

#[derive(FromForm)]
struct Opts {
    tunnel: String,
    key: String
}

//#[derive(Clone)]
struct IpAddress(String);

impl std::string::ToString for IpAddress {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for IpAddress {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let remote = if let Some(remote) = req.remote() {
            remote
        } else {
            return Outcome::Failure((Status::InternalServerError, ()));
        };

        Outcome::Success(IpAddress(remote.ip().to_string()))
    }
}


#[get("/update?<opts>")]
fn update(opts: Opts, ip: IpAddress) -> String {
    let app_key = if let Some(app_key) = env::var("APP_KEY").ok() {
        app_key
    } else {
        String::from("SeCrEtKeY")
    };

    if app_key == opts.key {
        let result = Command::new("/sbin/ip")
            .arg("tunnel")
            .arg("change")
            .arg(opts.tunnel)
            .arg("remote")
            .arg(ip.to_string())
            .output()
            .expect("Filed to execute command.");

        result.stderr.into_iter().map(|d| d as char).collect::<String>()
    } else {
        "false".to_string()
    }
}

fn main() {
    rocket::ignite().mount("/", routes![update]).launch();
}
