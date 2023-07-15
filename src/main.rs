
mod db;
use db::init_db;

use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::{ FromRow, Type};
use sqlx::{Pool, Postgres};

#[derive(Debug, PartialEq, Eq, Type, Serialize, Clone, Copy, Deserialize)]
#[sqlx(type_name = "todo_status_enum", rename_all = "lowercase")]
enum TodoStatusEnum {
    Open,
    Close,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Task {
    id: i64,
    cid: i64,
    #[serde(with = "ts_seconds")]
    ctime: DateTime<Utc>,
    mid: Option<i64>,
    #[serde(with = "ts_seconds_option")]
    mtime: Option<DateTime<Utc>>,
    title: String,

    status: TodoStatusEnum,
}

#[get("/")]
async fn get_root(data: web::Data<App_data>) -> impl Responder {
    return getTasks(data).await;
}

#[get("/task")]
async fn get_tasks(data: web::Data<App_data>) -> impl Responder {
    return getTasks(data).await;
}

async fn getTasks(data: web::Data<App_data>) -> impl Responder {
    let _content =
        sqlx::query_as::<_, Task>(r#"SELECT id,cid,ctime ,mid ,mtime ,title,status  from todo"#)
            .fetch_all(&data.db)
            .await
            .unwrap();
    return HttpResponse::Ok().json(_content);
}

#[get("/task/{task_id}")]
async fn get_task(path: web::Path<u32>, data: web::Data<App_data>) -> impl Responder {
    let id = path.into_inner() as i64;

    let _content = sqlx::query_as::<_, Task>(
        r#"SELECT id,cid,ctime ,mid ,mtime ,title,status  from todo where id=$1"#,
    )
    .bind(id)
    .fetch_all(&data.db)
    .await
    .unwrap();
    println!("{:?}", id);

    return HttpResponse::Ok().json(_content);
}

#[derive(Serialize, Deserialize)]
struct Task_init {
    cid: i64,
    title: String,
}



#[post("/task")]
async fn submitTask(req_body: web::Json<Task_init>, data: web::Data<App_data>) -> impl Responder {

    let _content = sqlx::query_as::<_, Task>(r#"insert into todo(cid,title) values($1,$2)"#)
        .bind(req_body.cid)
        .bind(req_body.title.clone())
        .fetch_all(&data.db)
        .await
        .unwrap();
    println!("{:?} inserted", _content);
    return HttpResponse::Ok().json(req_body);
}

#[delete("/task/{id}")]
async fn delete_task(path: web::Path<u32>, data: web::Data<App_data>) -> impl Responder {
    let id = path.into_inner() as i64;
    let _content = sqlx::query_as::<_, Task>(r#"delete from todo where id=$1"#)
        .bind(id)
        .fetch_all(&data.db)
        .await
        .unwrap();
    return HttpResponse::Ok().json(id);
}

// #[put("/task/{id}")]
async fn close_task(path: web::Path<u32>, data: web::Data<App_data>) -> impl Responder {
    let id = path.into_inner() as i64;
    let close = TodoStatusEnum::Close;
    let _content = sqlx::query_as::<_, Task>(r#"update todo set todo_status_enum=$1 where id=$2"#)
        .bind(close)
        .bind(id)
        .fetch_all(&data.db)
        .await
        .unwrap();
    return HttpResponse::Ok().json(id);
}

#[derive(Clone)]
struct App_data {
    db: Pool<Postgres>,
}

// #[tokio::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let app_data = web::Data::new(App_data {
    //     visit_count: Mutex::new(0),
    // });
    let db = init_db().await.unwrap();
    let app_state = App_data { db };

    println!("Running server");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(get_root)
            .service(get_task)
            .service(get_tasks)
            .service(submitTask)
            .service(delete_task)
    })
    .workers(4)
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
