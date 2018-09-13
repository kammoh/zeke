extern crate actix;
extern crate actix_web;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate openssl;
extern crate rexpect;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde;

mod app_state;
mod check_login;
mod ssh_session;

use actix_web::{http, middleware, fs, server, App, HttpRequest, Form, HttpResponse};
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService, RequestIdentity};
use rexpect::spawn_bash;
use rexpect::errors::*;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use app_state::*;
use check_login::*;
use ssh_session::*;

fn who(req: &HttpRequest<AppState>) -> String {
    format!("Hello {} host={} remote={} agent={:?}",
            req.identity().unwrap_or("Anonymous".to_owned()),
        req.peer_addr().unwrap().to_string(),
        req.connection_info().remote().unwrap_or("unknown"),
        req.headers().get(actix_web::http::header::USER_AGENT)
                .unwrap_or(&actix_web::http::header::HeaderValue::from_str("xxx").unwrap()),

    )
}

fn do_start(req: &HttpRequest<AppState>) -> String {
    do_ssh_repl("pi", "zeke.us.to").unwrap_or_else(|e| format!("Error: {:?}", e))
}

#[derive(Deserialize)]
pub struct LoginData {
    name: String,
    password: String,
}

fn login(
    (req, params): (HttpRequest<AppState>, Form<LoginData>),
) -> HttpResponse {
    let user = params.name.clone();
    let pass = params.password.clone();

    info!("login name={} password={}", &user, &pass);

//    if user == pass {
//        req.remember(user);
//        HttpResponse::Ok()
//            .body("OK")
//    } else {
        HttpResponse::Ok()
            .body("Failed")
//    }

}

fn logout(req: &HttpRequest<AppState>) -> HttpResponse {
    req.forget();
    HttpResponse::Found()
        .header("location", "/").finish()
}

fn not_found(req: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../public/404.html"))
}

fn start(req: &HttpRequest<AppState>) -> HttpResponse {
    if let Some(id) = req.identity() {
        HttpResponse::Ok()
            .body(do_start(req))
    } else {
        HttpResponse::Unauthorized()
            .header("location", "/")
            .finish()
    }
}

fn main() {
//    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let sys = actix::System::new("zeke");


    // load ssl keys
    let mut tls_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    tls_builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    tls_builder.set_certificate_chain_file("cert.pem").unwrap();



    let server = server::new(|| {
        let mut cookie_key = vec![0; 32];
        cookie_key[1] = 77;

        vec![
            App::with_state(AppState{ auth_state: AuthState::Off })
                .prefix("auth")
                .middleware(middleware::Logger::default())
                .middleware(IdentityService::new(
                    CookieIdentityPolicy::new(&cookie_key)
                        .name("auth")
                        .secure(false),
                ))
                .resource("/login", |r| {
                    r.method(http::Method::POST).with(login)
                })
                .resource("/logout", |r| r.f(logout))
                .resource("/who", |r| r.f(who))
                .resource("/start", |r| r.f(start))
            ,

            App::with_state(AppState{ auth_state: AuthState::Off })
                .prefix("/")
                // enable logger
                .middleware(middleware::Logger::default())

                .handler(
                    "/",
                    fs::StaticFiles::new("./public/").unwrap()
                        .index_file("login.html")
                        .default_handler(not_found)
                )
        ]
    });

    let listen_port = 8443;

    let listen_addr = "0.0.0.0";

    server.bind_ssl((listen_addr, listen_port), tls_builder)
        .expect("Can not start Zeke on given IP/Port")
        .start();

    info!("Started server at https://localhost:{}", listen_port);

    let _ = sys.run();
}