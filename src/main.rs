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

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;

mod types;
mod implementations;
mod parser;
mod gui;

use std::collections::LinkedList;
use std::fs::File;
use std::io::Write;
use std::io;
use std::error::Error;
use std::result::Result;

use types::*;
use parser::*;

/// returns true if an entry exists for the given course type in the given day
fn course_is_today(course_type: CourseType, day: &Day) -> bool {
    day.courses
        .borrow()
        .iter()
        .any(|course| course.course_type == course_type)
}

/// subfunction of distribute_courses
/// returns the left space (seats)
fn distribute_course(
    course: &Course,
    day: &Day,
    participants: &mut LinkedList<Group>,
    space_count: u8,
) -> u8 {
    let mut space_count = space_count;
    while space_count > 0 {
        space_count -= 1;
        let mut splitter = 0;
        for ref group in participants.iter() {
            if group.is_occupied(course, day) {
                splitter = splitter + 1;
            } else {
                break;
            }
        }
        let mut rest = participants.split_off(splitter); // the due to occupation skipped part
        let group = match rest.pop_front() {
            // get relevant group
            Some(group) => group,
            None => continue,
        };
        rest.push_back(group); // move the relevant group to the back
        participants.append(&mut rest); // reunite
        if let Some(group) = participants.back() {
            // get relevant group (old group was consumed by pushing)
            let mut course_participants = course.participants.borrow_mut();
            for student in group.participants.borrow_mut().iter_mut() {
                course_participants.push_back(student.clone());
            }
        }
    }
    space_count
}

/// Takes a list of Groups and distributes them among the course.
/// The distributed groups will be moved to the end of the given group list, so
/// that the next time, they will be distributed with the least priority
fn distribute_courses<'a, 'b>(
    course_type: CourseType,
    day: &'a Day,
    participants: &'b mut LinkedList<Group>,
) {
    if course_is_today(course_type, day) {
        match course_type {
            CourseType::Curriculum => {
                let courses = day.courses.borrow();
                if let Some(course) = courses
                    .iter()
                    .find(|ref course| course.course_type == CourseType::Curriculum)
                {
                    distribute_course(&course, day, participants, 1);
                }
            }
            CourseType::Exkurs => {
                let courses = day.courses.borrow();
                if let Some(course) = courses
                    .iter()
                    .find(|ref course| course.course_type == CourseType::Exkurs)
                {
                    distribute_course(&course, day, participants, 1);
                }
            }
            CourseType::Zahnersatz => {
                let courses = day.courses.borrow();
                if let Some(course) = courses.iter().find(|ref course| {
                    course.course_type == CourseType::Zahnersatz && course.beginning == 7
                }) {
                    distribute_course(
                        &course,
                        day,
                        participants,
                        (get_zahnersatz_seat_count() / 2 as u8),
                    );
                }
                if let Some(course) = courses.iter().find(|ref course| {
                    course.course_type == CourseType::Zahnersatz && course.beginning == 16
                }) {
                    distribute_course(
                        &course,
                        day,
                        participants,
                        (get_zahnersatz_seat_count() / 2 as u8),
                    );
                }
            }
            CourseType::Zahnerhalt => {
                let courses = day.courses.borrow();
                if let Some(course) = courses.iter().find(|ref course| {
                    course.course_type == CourseType::Zahnerhalt && course.beginning == 7
                }) {
                    distribute_course(&course, day, participants, get_zahnerhalt_seat_count());
                }
                if let Some(course) = courses.iter().find(|ref course| {
                    course.course_type == CourseType::Zahnerhalt && course.beginning == 16
                }) {
                    distribute_course(&course, day, participants, get_zahnerhalt_seat_count());
                }
            }
        }
    }
}

fn print_course<T: Write>(
    file: &mut T,
    week: &Week,
    course_type: CourseType,
    beginning: u8,
) -> Result<(), io::Error> {
    write!(file, "{} {}   ", course_type, beginning)?;
    for day_index in 0..5 {
        let current_day = &week.days[day_index];

        if let Some(course) =
            current_day.courses.borrow().iter().find(|ref course| {
                course.course_type == course_type && course.beginning == beginning
            }) {
            write!(file, "{}", StudentPrinter(&course.participants.borrow()))?;
        } else {
            write!(file, "{}", StudentPrinter(&LinkedList::new()))?;
        }
    }
    writeln!(file, "")?;
    Ok(())
}

///takes in the data, formats it so that it is humanly readable and writes it to the given Writer
fn generate_output<'a, T: Write>(file: &mut T, weeks: &'a Vec<Week>) -> Result<(), io::Error> {
    for current_week in weeks {
        writeln!(file, "KW {}", current_week.number)?;
        writeln!(file, "               Montag                                    Dienstag                                  Mittwoch                                  Donnerstag                                Freitag")?;
        print_course(file, &current_week, CourseType::Curriculum, 7)?;
        print_course(file, &current_week, CourseType::Exkurs, 7)?;
        print_course(file, &current_week, CourseType::Zahnerhalt, 7)?;
        print_course(file, &current_week, CourseType::Zahnerhalt, 16)?;
        print_course(file, &current_week, CourseType::Zahnersatz, 7)?;
        print_course(file, &current_week, CourseType::Zahnersatz, 16)?;
        writeln!(file, "")?;
        writeln!(file, "")?;
    }
    Ok(())
}

/// The MAIN function... very important
fn main() {
    gui::main();
    println!("---start---");
    let input = parse();
    println!("---parsed json file---");
    println!("{}", serde_json::to_string_pretty(&input).unwrap());
    let weeks = get_weeks(&input);
    println!("---parsed weeks---",);
    //println!("{:#?}", weeks);
    let students = get_students(&input);
    println!("---parsed students---",);
    for student in students.iter() {
        println!("{}", student)
    }
    let mut curriculum_groups = get_curriculum_groups(&input, &students);
    println!("---parsed curriculum groups---",);
    for group in curriculum_groups.iter() {
        println!("{}", group)
    }
    let mut exkurs_groups = get_exkurs_groups(&input, &students);
    println!("---parsed exkurs groups---",);
    for group in exkurs_groups.iter() {
        println!("{}", group)
    }
    let mut zahnersatz_groups = get_zahnersatz_groups(&students);
    println!("---parsed Zahnersatz groups---",);
    println!("first half:");
    for group in zahnersatz_groups.0.iter() {
        println!("{}", group)
    }
    println!("second half:");
    for group in zahnersatz_groups.1.iter() {
        println!("{}", group)
    }
    let mut zahnerhalt_groups = get_zahnerhalt_groups(&students);
    println!("---parsed Zahnerhalt groups---",);
    for current_week in weeks.iter() {
        println!("---process week {}---", current_week.number);
        for day_index in 0..5 {
            println!("---process day {}---", day_index);
            let current_day = &current_week.days[day_index];
            distribute_courses(CourseType::Curriculum, current_day, &mut curriculum_groups);
            distribute_courses(CourseType::Exkurs, current_day, &mut exkurs_groups);
            distribute_courses(
                CourseType::Zahnersatz,
                current_day,
                &mut zahnersatz_groups.0,
            );
            distribute_courses(
                CourseType::Zahnersatz,
                current_day,
                &mut zahnersatz_groups.1,
            );
            distribute_courses(CourseType::Zahnerhalt, current_day, &mut zahnerhalt_groups);
        }
    }
    let mut file = match File::create("Alban says.txt") {
        Err(why) => panic!(
            "couldn't create {}: {}",
            "Alban says.txt",
            why.description()
        ),
        Ok(file) => file,
    };
    let _ = generate_output(&mut std::io::stdout(), &weeks);
    if let Err(err) = generate_output(&mut file, &weeks) {
        println!("Unable to write output to file: {}", err.description());
    }
}
