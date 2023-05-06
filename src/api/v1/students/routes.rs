use super::create::create_student;
use super::edit::edit_student;
use super::get::get_student;
use super::list::list_student;

use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/students")
            .route("", web::post().to(create_student))
            .route("/{id}", web::get().to(get_student))
            .route("/{id}", web::put().to(edit_student))
            .route("", web::get().to(list_student)),
    );
}
