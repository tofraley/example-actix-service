use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Student {
    #[serde(skip_deserializing, default = "gen_id")]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub grade: u8,
}

pub fn gen_id() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Debug)]
pub enum StudentError {
    AddError,
}

pub struct MockRepo {
    pub students: Mutex<Vec<Student>>,
}

impl MockRepo {
    pub fn new() -> Self {
        MockRepo {
            students: Mutex::new(vec![
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
                },
            ]),
        }
    }


    pub fn get_students(&self) -> Vec<Student> {
        self.students.lock().unwrap().to_vec()
    }

    pub fn get_student_by_id(&self, id: &str) -> Option<Student> {
        let students = self.students.lock().unwrap();
        for student in &*students {
            if student.id == id.to_string() {
                return Some(student.clone());
            }
        }
        None
    }

    pub fn add_student(&self, student: Student) -> Option<Student> {
        let id = student.id.clone();
        self.students.lock().unwrap().push(student);
        self.get_student_by_id(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let repo: MockRepo = MockRepo::new();

        assert_eq!(repo.students[0].id, "123".to_string());
    }

    #[test]
    fn get_students_works() {
        let repo = MockRepo::new();
        let actual = repo.get_students();
        assert!(actual.len() == 2);
    }

    #[test]
    fn get_student_by_id_works() {
        let repo = MockRepo::new();
        let actual = repo.get_student_by_id("123");
        assert_eq!(actual, Some(&repo.students[0]));
    }
}
