use std::collections::LinkedList;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourseType {
    Curriculum,
    Exkurs,
    Zahnersatz,
    Zahnerhalt,
}

///
#[derive(Debug)]
pub struct Week {
    pub number: u64,
    pub days: [Day; 5], //Mo-Fr
}

#[derive(Default, Debug)]
pub struct Day {
    pub courses: RefCell<LinkedList<Course>>,
}

#[derive(Debug)]
pub struct Course {
    pub beginning: u8,
    pub course_type: CourseType,
    pub participants: RefCell<LinkedList<Rc<Student>>>,
}

#[derive(Debug)]
pub struct Student {
    pub number: u64,
}

pub struct StudentPrinter<'a>(pub &'a LinkedList<Rc<Student>>); // needed to print generic

#[derive(Debug)]

pub struct Group {
    pub group_type: CourseType,
    pub participants: RefCell<LinkedList<Rc<Student>>>,
}

pub trait Occupation<'a> {
    fn is_occupied(&self, course_type: CourseType, day: &Day) -> bool;
}
