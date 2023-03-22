use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "calculate average year experience a job need, please type CEO,CFO,VICE_PRESIDENT,JUNIOR"
}
#[get("/CEO")]
async fn ceo() -> &'static str {
    "calculate average year experience a job need: 11.10344827586207"
}
#[get("/CFO")]
async fn cfo() -> &'static str {
    "calculate average year experience a job need: 11.793893129770993"
}
#[get("/VICE_PRESIDENT")]
async fn vice() -> &'static str {
    "calculate average year experience a job need: 12.10759493670886"
}
#[get("/JUNIOR")]
async fn junior() -> &'static str {
    "calculate average year experience a job need: 12.987179487179487"
}
#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world).service(ceo).service(cfo).service(vice).service(junior);
    };

    Ok(config.into())
}