extern crate actix_web;

use actix_web::middleware::{Middleware, Started};
use actix_web::{http, HttpRequest, HttpResponse, Result};

use app_state::*;

#[derive(Default)]
pub struct AuthMiddleware;

impl Middleware<AppState> for AuthMiddleware {
    // We only need to hook into the `start` for this middleware.
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {

        if let AuthState::In(_) = req.state().auth_state {
            return Ok(Started::Done);
        }

        // Don't forward to /login if we are already on /login
        if req.path() == "/login" {
            return Ok(Started::Done);
        }

        Ok(Started::Response(
            HttpResponse::Found()
                .header(http::header::LOCATION, "/login")
                .finish(),
        ))
    }
}