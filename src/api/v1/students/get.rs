use actix_web::web::Path;

use actix_web::HttpResponse;

pub async fn get_student(path: Path<(i32,)>) -> HttpResponse {
    let student_id = path.0;
    HttpResponse::Ok().body(format!("Here is the student with the id {}", student_id))
}
