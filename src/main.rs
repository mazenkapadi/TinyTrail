use actix_web::{web, App, HttpServer};
use actix_files as fs;

mod routes;
mod firestore;
mod models;
mod utils;
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()>{
//     HttpServer::new(||{
//         App::new()
//             .service(fs::Files::new("/static", "./static").show_files_listing())
//             .service(web::scope("/api").configure(routes::config))
//             .service(fs::Files::new("/", "./src/templates").index_file("index.html"))
//     })
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }


#[tokio::main]
async fn main() {
    let id = "example123";
    let long_url = "https://example.com";
    let expiry = 1672531199; // Example timestamp

    match save_url(id, long_url, expiry).await {
        Ok(_) => println!("URL saved successfully!"),
        Err(e) => eprintln!("Error saving URL: {}", e),
    }
}
