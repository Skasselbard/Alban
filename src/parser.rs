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

use types::*;

pub fn get_weeks() -> Option<Vec<Week>> {
    let mut weeks = Vec::with_capacity(20);
    for i in 1..14 {
        let week = Week {
            number: i,
            days: {
                let mut ret: [Day; 5] = Default::default();
                for i in 0..5 {
                    ret[i] = Day {
                        //TODO: add courses conditionally
                        courses: {
                            RefCell::new({
                                let mut courses = LinkedList::new();
                                courses.push_back(Course {
                                    beginning: 7,
                                    course_type: CourseType::Curriculum,
                                    participants: RefCell::new(LinkedList::new()),
                                });
                                courses.push_back(Course {
                                    beginning: 7,
                                    course_type: CourseType::Exkurs,
                                    participants: RefCell::new(LinkedList::new()),
                                });
                                courses.push_back(Course {
                                    beginning: 7,
                                    course_type: CourseType::Zahnerhalt,
                                    participants: RefCell::new(LinkedList::new()),
                                });
                                courses.push_back(Course {
                                    beginning: 7,
                                    course_type: CourseType::Zahnersatz,
                                    participants: RefCell::new(LinkedList::new()),
                                });
                                courses.push_back(Course {
                                    beginning: 16,
                                    course_type: CourseType::Zahnerhalt,
                                    participants: RefCell::new(LinkedList::new()),
                                });
                                courses.push_back(Course {
                                    beginning: 16,
                                    course_type: CourseType::Zahnersatz,
                                    participants: RefCell::new(LinkedList::new()),
                                });
                                courses
                            })
                        },
                    }
                }
                ret
            },
        };
        weeks.push(week);
    }
    Some(weeks)
}

pub fn get_students<'a>() -> LinkedList<Rc<Student>> {
    let mut students = LinkedList::new();
    for i in 1..27 {
        let student = Rc::new(Student { number: i });
        students.push_back(student);
    }
    students
}

pub fn get_curriculum_groups<'a>(students: &LinkedList<Rc<Student>>) -> LinkedList<Group> {
    let mut groups = LinkedList::new();
    let mut students_iterator = students.iter();
    for _ in 0..5 {
        let group = Group {
            group_type: CourseType::Curriculum,
            participants: {
                let mut participants = LinkedList::new();
                for _ in 0..5 {
                    if let Some(student) = students_iterator.next() {
                        participants.push_back(student.clone());
                    }
                }
                RefCell::new(participants)
            },
        };
        groups.push_back(group);
    }
    groups
}

pub fn get_exkurs_groups<'a>(students: &LinkedList<Rc<Student>>) -> LinkedList<Group> {
    let mut groups = LinkedList::new();
    let mut students_iterator = students.iter();
    for _ in 0..13 {
        let group = Group {
            group_type: CourseType::Exkurs,
            participants: {
                let mut participants = LinkedList::new();
                for _ in 0..2 {
                    if let Some(student) = students_iterator.next() {
                        participants.push_back(student.clone());
                    }
                }
                RefCell::new(participants)
            },
        };
        groups.push_back(group);
    }
    groups
}

pub fn get_zahnersatz_groups<'a>(
    students: &LinkedList<Rc<Student>>,
) -> (LinkedList<Group>, LinkedList<Group>) {
    let mut first_half = LinkedList::new();
    let mut second_half = LinkedList::new();
    let mut position = 0;
    for student in students.iter() {
        let group = Group {
            group_type: CourseType::Zahnerhalt,
            participants: {
                let mut participants = LinkedList::new();
                participants.push_back(student.clone());
                RefCell::new(participants)
            },
        };
        if position < students.len() / (2 as usize) {
            first_half.push_back(group);
        } else {
            second_half.push_back(group)
        }
        position += 1;
    }
    (first_half, second_half)
}

pub fn get_zahnerhalt_groups<'a>(students: &LinkedList<Rc<Student>>) -> LinkedList<Group> {
    let mut groups = LinkedList::new();
    for student in students.iter() {
        let group = Group {
            group_type: CourseType::Zahnersatz,
            participants: {
                let mut participants = LinkedList::new();
                participants.push_back(student.clone());
                RefCell::new(participants)
            },
        };
        groups.push_back(group);
    }
    groups
}

pub fn get_zahnerhalt_seat_count() -> u8 {
    11
}

pub fn get_zahnersatz_seat_count() -> u8 {
    10
}
