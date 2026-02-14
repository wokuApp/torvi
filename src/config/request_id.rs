use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use uuid::Uuid;

pub struct RequestIdFairing;

#[rocket::async_trait]
impl Fairing for RequestIdFairing {
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let id = request
            .headers()
            .get_one("X-Request-Id")
            .map(String::from)
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        request.local_cache(|| id);
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let id = request.local_cache(|| Uuid::new_v4().to_string());
        response.set_raw_header("X-Request-Id", id.clone());
    }
}

pub fn init() -> RequestIdFairing {
    RequestIdFairing
}
