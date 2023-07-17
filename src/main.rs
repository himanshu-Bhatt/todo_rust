mod api;
mod db;
mod model;
use db::init_db;
use model::App_data;

use api::{change_title_status, delete_task, get_task, get_tasks, submitTask};

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = init_db().await.unwrap();
    let app_state = App_data { db };

    println!("Running server");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(get_tasks))
            .route("/task/", web::get().to(get_tasks))
            .route("/task", web::get().to(get_tasks))
            .route("/task/{id}", web::get().to(get_task))
            .route("/task/{id}/", web::get().to(get_task))
            .route("/task", web::post().to(submitTask))
            .route("/task/", web::post().to(submitTask))
            .route("/task/{id}", web::delete().to(delete_task))
            .route("/task/{id}/", web::delete().to(delete_task))
            .route("/task/{id}", web::patch().to(change_title_status))
            .route("/task/{id}/", web::patch().to(change_title_status))
    })
    .workers(4)
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
