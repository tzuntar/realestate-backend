use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::body::BoxBody;

use serde::{Serialize, Deserialize};

use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct Ticket {
    pub id: u32,
    pub author: String
}

#[derive(Debug, Serialize)]
pub struct ErrNoId {
    pub id: u32,
    pub err: String
}

impl Responder for Ticket {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}

impl Display for ErrNoId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ErrNoId {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::new(self.status_code())
            .set_body(BoxBody::new(body))
    }
}
