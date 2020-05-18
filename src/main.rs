use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use uuid::Uuid;
mod student;
use student::Student;

struct FakeDb {
    students: Vec<Student>,
}

fn create_mock_student() -> Student {
    Student {
        id: Uuid::new_v4().to_string(),
        first_name: "test".to_string(),
        last_name: "test".to_string(),
        grade: 4,
    }
}

async fn get_students(fake_db: web::Data<FakeDb>) -> impl Responder {
    let students = &fake_db.students;
    let json = serde_json::to_string(&students).unwrap();
    HttpResponse::Ok().content_type("application/json").body(json)
}

async fn get_student_by_id(fake_db: web::Data<FakeDb>) -> impl Responder {
    let student = &fake_db.students[0];
    let json = serde_json::to_string(&student).unwrap();
    HttpResponse::Ok().content_type("application/json").body(json)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let student1 = create_mock_student();
    let student2 = create_mock_student();
    let student3 = create_mock_student();
    let fake_db = web::Data::new(FakeDb {
        students: vec![student1, student2, student3],
    });
    let mut listenfd = ListenFd::from_env();
    let mut server =
        HttpServer::new(move || {
            // move fakeDb into the closure
            App::new()
                .app_data(fake_db.clone())
                .service(
                web::scope("/v1/students")
                    .route("/", web::get().to(get_students))
                    .route("/123", web::get().to(get_student_by_id)),
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

    fn get_fake_db() -> FakeDb {
        FakeDb {
            students: vec![
                            Student {
                                id: "123".to_string(),
                                first_name: "Taylor".to_string(),
                                last_name: "Testo".to_string(),
                                grade: 4,
                            },
                            Student {
                                id: "test2".to_string(),
                                first_name: "Kerry".to_string(),
                                last_name: "Testine".to_string(),
                                grade: 5,
                            },],
        }
    }

    #[actix_rt::test]
    async fn get_students_ok() {
        let fake_db = web::Data::new(get_fake_db());

        let mut app = test::init_service(
            App::new()
                .app_data(fake_db.clone())
                .route("/v1/students/", web::get().to(get_students))
        ).await;

        let req = test::TestRequest::get()
            .uri("/v1/students/")
            .to_request();
        let resp: Vec<Student> = test::read_response_json(&mut app, req).await;

        assert_eq!(fake_db.students, resp);
    }

    #[actix_rt::test]
    async fn get_student_by_id_ok() {
        let fake_db = web::Data::new(get_fake_db());

        let mut app = test::init_service(
            App::new()
                .app_data(fake_db.clone())
                .route("/v1/students/123", web::get().to(get_student_by_id))
        ).await;
        let req = test::TestRequest::get()
            .uri("/v1/students/123")
            .to_request();
        let resp: Student = test::read_response_json(&mut app, req).await;
        assert_eq!(resp, fake_db.students[0]);
    }
}
