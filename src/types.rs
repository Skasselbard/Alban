// Alban is a program to distribute dentistry students to their courses
// Copyright (C) 2017 Tom Meyer

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
