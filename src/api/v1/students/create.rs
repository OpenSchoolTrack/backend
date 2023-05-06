use actix_web::HttpResponse;

pub async fn create_student() -> HttpResponse {
    HttpResponse::Ok().body("Now you are creating a student")
}
