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

mod types;
mod implementations;

use std::collections::LinkedList;
use types::*;
use std::rc::Rc;
use std::cell::RefCell;

fn get_weeks() -> Option<Vec<Week>> {
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
                                courses.push_back(
                                    Course{
                                        beginning: 7,
                                        course_type: CourseType::Curriculum, 
                                        participants: RefCell::new(LinkedList::new())
                                    });
                                    courses.push_back(
                                    Course{
                                        beginning: 7,
                                        course_type: CourseType::Exkurs, 
                                        participants: RefCell::new(LinkedList::new())
                                    });
                                    courses.push_back(
                                    Course{
                                        beginning: 7,
                                        course_type: CourseType::Zahnerhalt, 
                                        participants: RefCell::new(LinkedList::new())
                                    });
                                    courses.push_back(
                                    Course{
                                        beginning: 7,
                                        course_type: CourseType::Zahnersatz, 
                                        participants: RefCell::new(LinkedList::new())
                                    });
                                    courses.push_back(
                                    Course{
                                        beginning: 16,
                                        course_type: CourseType::Zahnerhalt, 
                                        participants: RefCell::new(LinkedList::new())
                                    });
                                    courses.push_back(
                                    Course{
                                        beginning: 16,
                                        course_type: CourseType::Zahnersatz, 
                                        participants: RefCell::new(LinkedList::new())
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

fn get_students<'a>() -> LinkedList<Rc<Student>> {
    let mut students = LinkedList::new();
    for i in 1..27 {
        let student = Rc::new(Student { number: i });
        students.push_back(student);
    }
    students
}

fn get_curriculum_groups<'a>(students: &LinkedList<Rc<Student>>) -> LinkedList<Group> {
    let mut groups = LinkedList::new();
    let mut students_iterator = students.iter();
    for j in 0..5 {
        let group = Group {
            group_type: CourseType::Curriculum,
            participants: {
                let mut participants = LinkedList::new();
                for k in 0..5{
                    let student = students_iterator.next().expect("no students left").clone();
                    participants.push_back(student);
                }
                RefCell::new(participants)
            },
        };
        groups.push_back(group);
    }
    groups
}

fn get_exkurs_groups<'a>(students: &LinkedList<Rc<Student>>) -> LinkedList<Group> {
    let mut groups = LinkedList::new();
    let mut students_iterator = students.iter();
        for j in 0..13 {
            let group = Group {
                group_type: CourseType::Curriculum,
                participants: {
                    let mut participants = LinkedList::new();
                    for k in 0..2{
                        let student = students_iterator.next().expect("no students left").clone();
                        participants.push_back(student);
                    }
                    RefCell::new(participants)
                },
            };
            groups.push_back(group);
        }
    groups
}

fn course_is_today(course_type: CourseType, week: &Week) -> bool {
    true
}

fn distribute_course(
    course: &Course,
    day: & Day,
    participants: & mut LinkedList<Group>,
){
    let mut splitter = 0;
    for ref group in participants.iter() {
        if group.is_occupied(course, day) {
            splitter = splitter + 1;
        } else {
            break;
        }
    }
    let mut rest = participants.split_off(splitter);// the due to occupation skipped part 
    let group = rest.pop_front().unwrap();// get relevant group
    rest.push_back(group);// move the relevant group to the back
    participants.append(&mut rest);// reunite
    let group = participants.back().unwrap();// get relevant group (old group was consumed b pushing)
    let mut course_participants = course.participants.borrow_mut();
    for student in group.participants.borrow_mut().iter_mut() {
        course_participants.push_back(student.clone());
    }
}

fn distribute_courses<'a, 'b>(
    course_type: CourseType,
    week: &Week,
    day: &'a Day,
    participants: &'b mut LinkedList<Group>,
) {
    if course_is_today(course_type, week) {
        match course_type {
            CourseType::Curriculum => {
                let courses = day.courses.borrow();
                let course = courses.iter().find(|ref course|course.course_type == CourseType::Curriculum).unwrap();
                distribute_course(&course, day, participants);
            },
            CourseType::Exkurs => {
                let courses = day.courses.borrow();
                let course = courses.iter().find(|ref course|course.course_type == CourseType::Exkurs).unwrap();
                distribute_course(&course, day, participants);
            },
            CourseType::Zahnersatz => {
                let courses = day.courses.borrow();
                let course = courses.iter().find(|ref course|course.course_type == CourseType::Zahnersatz).unwrap();
                distribute_course(&course, day, participants);
            },
            CourseType::Zahnerhalt => {
                let courses = day.courses.borrow();
                let course = courses.iter().find(|ref course|course.course_type == CourseType::Zahnerhalt).unwrap();
                distribute_course(&course, day, participants);
            },
        }
    }
}

fn generate_output<'a>(weeks: &'a Vec<Week>) {
    for current_week in weeks {
        println!("KW {}", current_week.number);
        println!("_____________Montag_____________Dienstag________Mittwoch________Donnerstag______Freitag");
        for course_type in CourseType::variants() {
            print!("{}", course_type);
            print!("___");
            for day_index in 0..5 {
                let current_day = &current_week.days[day_index];
                for course in current_day.courses.borrow().iter() {
                    if course.course_type == *course_type{
                        print!("{}", StudentPrinter(&course.participants.borrow()));
                    }
                }
                print!("\t\t");
            }
            println!("");
        }
        println!("");
        println!("");
    }
}

fn main() {
    println!("---start---");
    let weeks = get_weeks().expect("Unable to parse weeks");
    println!("---parsed weeks---", );
    let mut students = get_students();
    println!("---parsed students---", );
    let mut curriculum_groups = get_curriculum_groups(&students);
    println!("---parsed curriculum groups---", );
    let mut exkurs_groups = get_exkurs_groups(&students);
    println!("---parsed exkurs groups---", );
    for current_week in weeks.iter() {
        println!("---process week {}---", current_week.number);
        for day_index in 0..5 {
            println!("---process day {}---", day_index);
            let current_day = &current_week.days[day_index];
            distribute_courses(
                CourseType::Curriculum,
                &current_week,
                current_day,
                &mut curriculum_groups,
            );
            distribute_courses(
                CourseType::Exkurs,
                &current_week,
                current_day,
                &mut exkurs_groups,
            );
            // distribute_courses(
            //     CourseType::Zahnersatz,
            //     &current_week,
            //     current_day,
            //     &mut students,
            // );
            // distribute_courses(
            //     CourseType::Zahnerhalt,
            //     &current_week,
            //     current_day,
            //     &mut students,
            // );
        }
    }
    generate_output(&weeks);
}
