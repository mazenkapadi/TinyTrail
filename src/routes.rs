// use actix_web::{web, HttpResponse, Responder};
// use crate::{firestore, models, utils};
// use chrono::Utc;
//
// pub fn config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/shorten")
//             .route(web::post().to(shorten_url)),
//     )
//         .service(
//             web::resource("/resolve/{id}")
//                 .route(web::get().to(resolve_url)),
//         );
// }
//
// async fn shorten_url(data: web::Json<models::ShortenRequest>) -> impl Responder {
//     let short_id = utils::generate_short_id();
//     let expiry = utils::calculate_expiry(data.validity);
//
//     match firestore::save_url(&short_id, &data.long_url, expiry).await {
//         Ok(_) => HttpResponse::Ok().json(models::ShortenResponse {
//             short_url: format!("http://localhost:8080/resolve/{}", short_id),
//         }),
//         Err(_) => HttpResponse::InternalServerError().body("Failed to save URL"),
//     }
// }
//
// async fn resolve_url(path: web::Path<String>) -> impl Responder {
//     let id = path.into_inner(); // Extract the ID from the path
//     match firestore::get_url(&id).await {
//         Ok(Some(url)) => HttpResponse::Ok().body(url), // Return the original URL
//         Ok(None) => HttpResponse::NotFound().body("URL not found"), // If no matching URL is found
//         Err(_) => HttpResponse::InternalServerError().body("Internal error"), // Handle Firestore errors
//     }
// }




use actix_web::{web, HttpResponse, Responder};
use crate::{firestore, models, utils};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/shorten")
            .route(web::post().to(shorten_url)),
    )
        .service(
            web::resource("/resolve/{id}")
                .route(web::get().to(resolve_url)),
        );
}

async fn shorten_url(data: web::Json<models::ShortenRequest>) -> impl Responder {
    let short_id = utils::generate_short_id();
    let expiry = utils::calculate_expiry(data.validity);

    match firestore::save_url(&short_id, &data.long_url, expiry).await {
        Ok(_) => HttpResponse::Ok().json(models::ShortenResponse {
            short_url: format!("http://localhost:8080/resolve/{}", short_id),
        }),
        Err(_) => HttpResponse::InternalServerError().body("Failed to save URL"),
    }
}

async fn resolve_url(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    match firestore::get_url(&id).await {
        Ok(Some(url)) => HttpResponse::Ok().body(url),
        Ok(None) => HttpResponse::NotFound().body("URL not found"),
        Err(_) => HttpResponse::InternalServerError().body("Internal error"),
    }
}