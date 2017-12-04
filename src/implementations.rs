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
use std::slice::Iter;

impl<'a> Day {
    fn has_course(&self) -> bool {
        self.courses.borrow().is_empty()
    }
}

impl<'a, 'b> Occupation<'a> for Group {
    fn is_occupied(&self, course_type: CourseType, day: &Day) -> bool {
        for student in self.participants.borrow().clone() {
            if student.is_occupied(course_type, day) {
                return true;
            }
        }
        false
    }
}

impl<'a> Occupation<'a> for Student {
    fn is_occupied(&self, course_type: CourseType, day: &Day) -> bool {
        //TODO: implement is occupied
        false
    }
}

impl CourseType {
    pub fn variants() -> Iter<'static, CourseType> {
        static VARIANTS: &'static [CourseType] = &[
            CourseType::Curriculum,
            CourseType::Exkurs,
            CourseType::Zahnerhalt,
            CourseType::Zahnersatz,
        ];
        VARIANTS.iter()
    }
}

impl fmt::Display for CourseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CourseType::Curriculum => write!(f, "Curriculum"),
            CourseType::Exkurs => write!(f, "Exkurs____"),
            CourseType::Zahnerhalt => write!(f, "Zahnerhalt"),
            CourseType::Zahnersatz => write!(f, "Zahnersatz"),
        }
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},", self.number)
    }
}

impl<'a> fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        for ref student in self.participants.borrow().iter() {
            result = write!(f, "{},", student)
        }
        result
    }
}

impl<'a> fmt::Display for StudentPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        for ref student in self.0.iter() {
            result = write!(f, "{},", student.number)
        }
        result
    }
}
