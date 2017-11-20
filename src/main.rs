// Alban is a program should distribute dentistry students to their courses
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

enum CourseType {
    Corriculum,
    Exkurs,
    Zahnersatz,
    Zahnerhalt,
}

///
struct Week {
    number: u64,
    days: [Day; 5], //Mo-Fr
}

#[derive(Default)]
struct Day {
    courses: LinkedList<Course>,
}

struct Course {
    beginning: u8,
    courseType: CourseType,
}

struct Student {
    number: u64,
}

struct Group {
    groupType: CourseType,
    participants: LinkedList<Student>,
}

impl Day {
    fn hasCourse(&self) -> bool {
        self.courses.is_empty()
    }
}

fn get_weeks() -> Option<Vec<Week>> {
    let mut weeks = Vec::with_capacity(20);
    for i in 1..14 {
        let week = Week {
            number: i,
            days: {
                let mut ret: [Day; 5] = Default::default();
                for i in 0..4 {
                    ret[i] = Day {
                        courses: LinkedList::new(),
                    }
                }
                ret
            },
        };
        weeks.push(week);
    }
    Some(weeks)
}

fn get_students() -> Option<LinkedList<Student>> {
    let mut students = LinkedList::new();
    for i in 1..26 {
        students.push_back({ Student { number: i } });
    }
    Some(students)
}

fn get_curriculum_groups(students: LinkedList<Student>) -> LinkedList<Group> {
    let mut curriculum_groups = LinkedList::new();
    let mut students = students.into_iter();
    for i in 0..4 {
        for j in 0..4 {
            let group = Group {
                groupType: CourseType::Corriculum,
                participants: {
                    let mut participants = LinkedList::new();
                    participants.push_back(students.next().expect("no students left"));
                    participants.push_back(students.next().expect("no students left"));
                    participants
                },
            };
            curriculum_groups.push_back(group);
        }
    }
    curriculum_groups
}

fn main() {
    println!("Hello");
    let weeks = get_weeks().expect("Unable to parse weeks");
    for currentWeek in weeks {
        for dayIndex in 0..5 {
            let currentDay = currentWeek.days[dayIndex];
        }
    }
    println!("And bye.");
}
