use std::sync::Mutex;
use actix_web::{App, HttpServer};
use actix_web::web;

use crate::models::ticket::Ticket;

mod config;
mod routes;
mod models;
mod service;

mod schema;

struct AppState {
    tickets: Mutex<Vec<Ticket>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        tickets: Mutex::new(vec![
            Ticket {
                id: 1,
                author: String::from("John Doe"),
            },
            Ticket {
                id: 2,
                author: String::from("Patrick Star"),
            },
        ])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web::resource("/tickets")
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}