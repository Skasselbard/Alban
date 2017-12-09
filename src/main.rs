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
mod parser;

use std::collections::LinkedList;
use types::*;
use parser::*;

fn course_is_today(course_type: CourseType, week: &Week) -> bool {
    //TODO: implement
    true
}


/// returns the left space (seats)
fn distribute_course(
    course: &Course,
    day: & Day,
    participants: & mut LinkedList<Group>,
    space_count: u8
)->u8{
    let mut space_count = space_count;
    while space_count > 0{
        space_count -= 1;
        let mut splitter = 0;
        for ref group in participants.iter() {
            if group.is_occupied(course, day) {
                splitter = splitter + 1;
            } else {
                break;
            }
        }
        let mut rest = participants.split_off(splitter);// the due to occupation skipped part 
        let group = match rest.pop_front(){// get relevant group
            Some(group) => group,
            None => continue
        };
        rest.push_back(group);// move the relevant group to the back
        participants.append(&mut rest);// reunite
        if let Some(group) = participants.back(){// get relevant group (old group was consumed b pushing)
            let mut course_participants = course.participants.borrow_mut();
            for student in group.participants.borrow_mut().iter_mut() {
                course_participants.push_back(student.clone());
            }
        }
    }
    space_count
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
                distribute_course(&course, day, participants, 1);
            },
            CourseType::Exkurs => {
                let courses = day.courses.borrow();
                let course = courses.iter().find(|ref course|course.course_type == CourseType::Exkurs).unwrap();
                distribute_course(&course, day, participants, 1);
            },
            CourseType::Zahnersatz => {
                let courses = day.courses.borrow();
                let course = courses.iter().find(
                    |ref course|
                    course.course_type == CourseType::Zahnersatz &&
                    course.beginning == 7
                    ).unwrap();
                distribute_course(&course, day, participants, (get_zahnersatz_seat_count()/2 as u8));
                let course = courses.iter().find(
                    |ref course|
                    course.course_type == CourseType::Zahnersatz &&
                    course.beginning == 16
                    ).unwrap();
                distribute_course(&course, day, participants, (get_zahnersatz_seat_count()/2 as u8));
            },
            CourseType::Zahnerhalt => {
                let courses = day.courses.borrow();
                let course = courses.iter().find(
                    |ref course|
                    course.course_type == CourseType::Zahnerhalt &&
                    course.beginning == 7
                    ).unwrap();
                distribute_course(&course, day, participants, get_zahnerhalt_seat_count());
                let course = courses.iter().find(
                    |ref course|
                    course.course_type == CourseType::Zahnerhalt &&
                    course.beginning == 16
                    ).unwrap();
                distribute_course(&course, day, participants, get_zahnerhalt_seat_count());
            },
        }
    }
}

fn print_course(week: &Week, course_type: CourseType, beginning:u8){
    print!("{} {}   ", course_type, beginning);
    for day_index in 0..5 {
        let current_day = &week.days[day_index];
        for course in current_day.courses.borrow().iter() {
            if course.course_type == course_type && course.beginning == beginning{
                print!("{}", StudentPrinter(&course.participants.borrow()));
            }
        }
        print!("\t\t");
    }
    println!("");
}

fn generate_output<'a>(weeks: &'a Vec<Week>) {
    for current_week in weeks {
        println!("KW {}", current_week.number);
        println!("               Montag           Dienstag        Mittwoch        Donnerstag      Freitag");
        print_course(&current_week, CourseType::Curriculum, 7);
        print_course(&current_week, CourseType::Exkurs, 7);
        print_course(&current_week, CourseType::Zahnerhalt, 7);
        print_course(&current_week, CourseType::Zahnerhalt, 16);
        print_course(&current_week, CourseType::Zahnersatz, 7);
        print_course(&current_week, CourseType::Zahnersatz, 16);
        println!("");
        println!("");
    }
}

fn main() {
    println!("---start---");
    let weeks = get_weeks().expect("Unable to parse weeks");
    println!("---parsed weeks---", );
    let students = get_students();
    println!("---parsed students---", );
    for student in students.iter(){
        println!("{}", student)
    }
    let mut curriculum_groups = get_curriculum_groups(&students);
    println!("---parsed curriculum groups---", );
    for group in curriculum_groups.iter(){
        println!("{}", group)
    }
    let mut exkurs_groups = get_exkurs_groups(&students);
    println!("---parsed exkurs groups---", );
    for group in exkurs_groups.iter(){
        println!("{}", group)
    }
    let mut zahnersatz_groups = get_zahnersatz_groups(&students);
    println!("---parsed Zahnersatz groups---", );
    println!("first half:");
    for group in zahnersatz_groups.0.iter(){
        println!("{}", group)
    }
    println!("second half:");
    for group in zahnersatz_groups.1.iter(){
        println!("{}", group)
    }
    let mut zahnerhalt_groups = get_zahnerhalt_groups(&students);
    println!("---parsed Zahnerhalt groups---", );
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
            distribute_courses(
                CourseType::Zahnersatz,
                &current_week,
                current_day,
                &mut zahnersatz_groups.0,
            );
            distribute_courses(
                CourseType::Zahnersatz,
                &current_week,
                current_day,
                &mut zahnersatz_groups.1,
            );
            distribute_courses(
                CourseType::Zahnerhalt,
                &current_week,
                current_day,
                &mut zahnerhalt_groups,
            );
        }
    }
    generate_output(&weeks);
}
