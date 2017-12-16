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

use types::*;
use std::fmt;
use std::rc::Rc;
use std::ops::Deref;


impl<'a, 'b> Occupation<'a> for Group {
    fn is_occupied(&self, course: &Course, day: &Day) -> bool {
        for student in self.participants.borrow().clone() {
            if student.is_occupied(course, day) {
                return true;
            }
        }
        false
    }
}

impl<'a> Occupation<'a> for Student {
    fn is_occupied(&self, course: &Course, day: &Day) -> bool {
        for ref course in day.courses
            .borrow()
            .iter()
            .filter(|ref x| x.beginning == course.beginning)
        //all courses with the same starting time
        {
            for student in course.participants.borrow().iter() {
                if Rc::deref(student) == self {
                    return true;
                }
            }
        }
        false
    }
}

impl fmt::Display for CourseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CourseType::Curriculum => write!(f, "Curriculum"),
            CourseType::Exkurs => write!(f, "Exkurs    "),
            CourseType::Zahnerhalt => write!(f, "Zahnerhalt"),
            CourseType::Zahnersatz => write!(f, "Zahnersatz"),
        }
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        if self.number < 10 {
            result = write!(f, " ");
        }
        if let Ok(()) = result {
            result = write!(f, "{},", self.number)
        }
        result
    }
}

impl<'a> fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        let participants = self.participants.borrow();
        let mut participants_iterator = participants.iter();
        for _ in 0..14 {
            if let Some(student) = participants_iterator.next() {
                result = write!(f, "{}", student);
            } else {
                result = write!(f, "   ");
            }
            if let Err(_) = result {
                break;
            }
        }
        result
    }
}

impl<'a> fmt::Display for StudentPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        let participants = self.0;
        let mut participants_iterator = participants.iter();
        for _ in 0..14 {
            if let Some(student) = participants_iterator.next() {
                result = write!(f, "{}", student);
            } else {
                result = write!(f, "   ");
            }
            if let Err(_) = result {
                break;
            }
        }
        result
    }
}
