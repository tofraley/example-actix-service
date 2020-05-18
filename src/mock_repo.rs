use crate::student::Student;

pub struct MockRepo {
    students: Vec<Student>,
}

impl MockRepo {
    pub fn new() -> Self {
        MockRepo {
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
                },
            ],
        }
    }

    pub fn get_students(&self) -> &Vec<Student> {
        &self.students
    }

    pub fn get_student_by_id(&self, id: &str) -> Option<&Student> {
        self.students
             .iter()
             .find(|&x| x.id == id.to_string())
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
