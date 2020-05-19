use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use student_service::{MockRepo, Student};

async fn get_students(repository: web::Data<MockRepo>) -> impl Responder {
    let students = repository.get_students();
    let json = serde_json::to_string(&students).unwrap();
    HttpResponse::Ok().content_type("application/json").body(json)
}

async fn get_student_by_id(id: web::Path<String>,
                           repository: web::Data<MockRepo>)
                           -> impl Responder {

    match repository.get_student_by_id(&id) {
        Some(student) => HttpResponse::Ok().json(student),
        None => HttpResponse::NotFound().body("Student not found.")
    }
}

async fn add_student(student: web::Json<Student>, repository: web::Data<MockRepo>) -> impl Responder {
    match repository.add_student(student.into_inner()) {
        Some(student) => HttpResponse::Ok().json(student),
        None => HttpResponse::InternalServerError().body("Problem adding student")
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let repository= web::Data::new(MockRepo::new());
    let mut listenfd = ListenFd::from_env();
    let mut server =
        HttpServer::new(move || {
            // move MockRepo into the closure
            App::new()
                .app_data(repository.clone())
                .service(
                web::scope("/v1/students")
                    .route("/", web::get().to(get_students))
                    .route("/", web::post().to(add_student))
                    .route("/{id}", web::get().to(get_student_by_id)),
            )}
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use student_service::Student;

    #[actix_rt::test]
    async fn get_students_ok() {
        let repository= web::Data::new(MockRepo::new());

        let mut app = test::init_service(
            App::new()
                .app_data(repository.clone())
                .route("/v1/students/", web::get().to(get_students))
        ).await;

        let req = test::TestRequest::get()
            .uri("/v1/students/")
            .to_request();
        let resp: Vec<Student> = test::read_response_json(&mut app, req).await;

        assert_eq!(repository.get_students(), &resp);
    }

    #[actix_rt::test]
    async fn get_student_by_id_ok() {
        let repository= web::Data::new(MockRepo::new());

        let mut app = test::init_service(
            App::new()
                .app_data(repository.clone())
                .route("/v1/students/123", web::get().to(get_student_by_id))
        ).await;
        let req = test::TestRequest::get()
            .uri("/v1/students/123")
            .to_request();
        let resp: Student = test::read_response_json(&mut app, req).await;
        let expected = repository.get_students();
        assert_eq!(resp, expected[0]);
    }
}
