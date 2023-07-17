use crate::model::{App_data, Task, Task_init, Task_patch, TodoStatusEnum};

use actix_web::{web, HttpResponse, Responder};

pub async fn get_tasks(data: web::Data<App_data>) -> impl Responder {
    let _content = sqlx::query_as::<_, Task>(
        r#"SELECT id,cid,ctime ,mid ,mtime ,title,status  from todo order by ctime desc"#,
    )
    .fetch_all(&data.db)
    .await
    .unwrap();
    return HttpResponse::Ok().json(_content);
}
pub async fn get_task(path: web::Path<u32>, data: web::Data<App_data>) -> impl Responder {
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

pub async fn submit_task(
    req_body: web::Json<Task_init>,
    data: web::Data<App_data>,
) -> impl Responder {
    let _content = sqlx::query_as::<_, Task>(r#"insert into todo(cid,title) values($1,$2)"#)
        .bind(req_body.cid)
        .bind(req_body.title.clone())
        .fetch_all(&data.db)
        .await
        .unwrap();
    println!("{:?} inserted", _content);
    return HttpResponse::Ok().json(req_body);
}

pub async fn delete_task(path: web::Path<u32>, data: web::Data<App_data>) -> impl Responder {
    let id = path.into_inner() as i64;
    let _content = sqlx::query_as::<_, Task>(r#"delete from todo where id=$1"#)
        .bind(id)
        .fetch_all(&data.db)
        .await
        .unwrap();
    return HttpResponse::Ok().json(id);
}

pub async fn change_title_status(
    path: web::Path<i64>,
    patch_todo: web::Json<Task_patch>,
    data: web::Data<App_data>,
) -> impl Responder {
    let id = path.into_inner();
    let Task_patch { title, status } = patch_todo.into_inner();

    if let Some(title) = title {
        let res = sqlx::query_as::<_, Task>(r#"update todo set title=$1 where id=$2 returning *"#)
            .bind(title)
            .bind(id)
            .fetch_all(&data.db)
            .await
            .unwrap();
        return HttpResponse::Ok().json(res);
    } else {
        if let Some(_status) = status {
            let _status = get_status_from_string(_status);

            let res =
                sqlx::query_as::<_, Task>(r#"update todo set status=$1 where id=$2 returning *"#)
                    .bind(_status)
                    .bind(id)
                    .fetch_all(&data.db)
                    .await
                    .unwrap();
            return HttpResponse::Ok().json(res);
        }
    }

    return HttpResponse::NoContent().json("");
}

pub fn get_status_from_string(status: String) -> TodoStatusEnum {
    if status == "Close" || status == "close" {
        return TodoStatusEnum::Close;
    } else {
        return TodoStatusEnum::Open;
    }
}
