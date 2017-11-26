use std::collections::LinkedList;

#[derive(Clone, Copy)]
pub enum CourseType {
    Corriculum,
    Exkurs,
    Zahnersatz,
    Zahnerhalt,
}

///
pub struct Week {
    pub number: u64,
    pub days: [Day; 5], //Mo-Fr
}

#[derive(Default)]
pub struct Day {
    pub courses: LinkedList<Course>,
}

pub struct Course {
    pub beginning: u8,
    pub courseType: CourseType,
}

#[derive(Debug)]
pub struct Student {
    pub number: u64,
}

pub struct Group {
    pub groupType: CourseType,
    pub participants: LinkedList<Student>,
}

pub trait Occupation {
    fn is_occupied(&self, course_type: CourseType, day: &Day) -> bool;
}
