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

extern crate serde_json;


use std::collections::LinkedList;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::error::Error;
use serde_json::error::Category;

use types::*;

/// Parses "input.json" in the current working directory and reports possible errors on std out
pub fn parse() -> JsonData {
    match File::open("./input.json") {
        Err(error) => {
            panic!("Unable to open json file: {}", error.description());
        }
        Ok(file) => match serde_json::from_reader(file) {
            Err(error) => {
                println!("{}", error.description());
                println!("line: {}, column {}", error.line(), error.column());
                match error.classify() {
                    Category::Io => {println!("IOError: unable to read stream")}
                    Category::Syntax => {println!("SyntaxError: file has malformed JSON. Did you miss or add brackets, colons etc?")}
                    Category::Data => {println!("DataError: parsed type does not match the expected type. Did you miss or add \"'s or confused arrays and objects?\nIf this error occurs on the last line, you might have misspelled a key?")}
                    Category::Eof => {println!("EOFError: premature end of file")}
                }
                panic!("Failed to parse json file");
            }
            Ok(data) => data,
        },
    }
}

/// Generates all weeks and its children.
/// Every day gets all courses unless it is a parsed holiday. Then it gets no course.
/// Additionally in a week with at least one holiday, there will be no Curriculum course.
/// all courses will have an empty list of participants assigned to them.
pub fn get_weeks(parsed_data: &JsonData) -> Vec<Week> {
    let mut weeks = Vec::with_capacity(20);
    let start = parsed_data.wochen.kwAnfang;
    let end = parsed_data.wochen.kwEnde + 1;
    let parsed_feiertags = &parsed_data.feiertage;
    if start > end {
        panic!("first week is after the last week");
    }
    for i in start..end {
        let week = Week {
            number: i,
            days: {
                let mut ret: [Day; 5] = Default::default();
                for j in 0..5 {
                    ret[j] = Day {
                        courses: {
                            RefCell::new({
                                let mut courses = LinkedList::new();
                                if let Some(entry) = parsed_feiertags
                                    .iter()
                                    .find(|&feiertag_entry| feiertag_entry.woche == i as u64)
                                {
                                    if let None =
                                        entry.tage.iter().find(|&&day| day - 1 == j as u64)
                                    {
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
                                    }
                                } else {
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
                                }
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
    weeks
}

/// Generates the list of students (a consecutive list of numbers, because there is
/// nothing more of importance to a student)
pub fn get_students<'a>(parsed_data: &JsonData) -> LinkedList<Rc<Student>> {
    let student_count = parsed_data.studentenAnzahl + 1;
    let mut students = LinkedList::new();
    for i in 1..student_count {
        let student = Rc::new(Student { number: i });
        students.push_back(student);
    }
    students
}

/// parse and generate
pub fn get_curriculum_groups<'a>(
    parsed_data: &JsonData,
    students: &LinkedList<Rc<Student>>,
) -> LinkedList<Group> {
    let parsed_groups = &parsed_data.curriculumGruppen;
    let mut groups = LinkedList::new();
    for parsed_group in parsed_groups {
        let group = Group {
            group_type: CourseType::Curriculum,
            participants: {
                let mut participants = LinkedList::new();
                for student_number in parsed_group {
                    for student in students {
                        if student.number == *student_number {
                            participants.push_back(student.clone());
                            break;
                        }
                    }
                }
                RefCell::new(participants)
            },
        };
        groups.push_back(group);
    }
    groups
}

/// parse and generate
pub fn get_exkurs_groups<'a>(
    parsed_data: &JsonData,
    students: &LinkedList<Rc<Student>>,
) -> LinkedList<Group> {
    let parsed_groups = &parsed_data.exkursGruppen;
    let mut groups = LinkedList::new();
    for parsed_group in parsed_groups {
        let group = Group {
            group_type: CourseType::Exkurs,
            participants: {
                let mut participants = LinkedList::new();
                for student_number in parsed_group {
                    for student in students {
                        if student.number == *student_number {
                            participants.push_back(student.clone());
                            break;
                        }
                    }
                }
                RefCell::new(participants)
            },
        };
        groups.push_back(group);
    }
    groups
}

/// parse and generate
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

/// parse and generate
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

/// given by the client (available work stations)
pub fn get_zahnerhalt_seat_count() -> u8 {
    11
}

/// given by the client (available work stations)
pub fn get_zahnersatz_seat_count() -> u8 {
    10
}
