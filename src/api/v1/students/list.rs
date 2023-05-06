use actix_web::HttpResponse;

pub async fn list_student() -> HttpResponse {
    HttpResponse::Ok().body("here is the list of students")
}
