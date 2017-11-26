//mod types;

use types::*;
impl Day {
    fn hasCourse(&self) -> bool {
        self.courses.is_empty()
    }
}

impl Occupation for Group {
    fn is_occupied(&self, course_type: CourseType, day: &Day) -> bool {
        for student in &(self.participants) {
            if student.is_occupied(course_type, day) {
                return false;
            }
        }
        true
    }
}

impl Occupation for Student {
    fn is_occupied(&self, course_type: CourseType, day: &Day) -> bool {
        false
    }
}
