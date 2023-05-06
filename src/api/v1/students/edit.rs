use actix_web::{web::Path, HttpResponse};

pub async fn edit_student(path: Path<(i32,)>) -> HttpResponse {
    let student_id = path.0;
    HttpResponse::Ok().body(format!("You are now editing student {}", student_id))
}
