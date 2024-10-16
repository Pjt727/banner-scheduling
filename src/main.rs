#![allow(dead_code)]
use serde::Deserialize;
// use serde_json::value::Index;
// use serde_json::Result;
// use std::cmp::Ordering;
use std::collections::HashMap;
// use std::env;
use std::fs::File;
// use std::hash::{Hash, Hasher};
use std::fmt;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
struct MeetingTime {
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
    #[serde(rename = "endTime")]
    end_time: Option<String>,
    #[serde(rename = "beginTime")]
    start_time: Option<String>,
}

#[derive(Deserialize, Debug)]
struct MeetingsFaculty {
    #[serde(rename = "meetingTime")]
    meeting_time: MeetingTime,
}

#[derive(Deserialize, Debug)]
struct Section {
    id: u32,
    term: String,
    #[serde(rename = "courseNumber")]
    course_number: String,
    #[serde(rename = "subject")]
    subject: String,
    #[serde(rename = "sequenceNumber")]
    sequence_number: String,
    #[serde(rename = "courseTitle")]
    course_title: String,
    #[serde(rename = "seatsAvailable")]
    seats_available: i16,
    #[serde(rename = "openSection")]
    open_section: bool,
    #[serde(rename = "meetingsFaculty")]
    meeting_faculty: Vec<MeetingsFaculty>,
    #[serde(rename = "creditHourLow")]
    credits: u8,
    #[serde(rename = "subjectCourse")]
    subject_course: String,
}

impl Section {
    fn has_tues_thurs(&self) -> bool {
        let mut has_monday = false;
        let mut has_tues = false;
        for fac in self.meeting_faculty.iter() {
            has_monday = has_monday || fac.meeting_time.monday;
            has_tues = has_tues || fac.meeting_time.monday;
        }
        return has_monday && has_tues;
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut days: Vec<String> = vec![];
        let mut day_lines: Vec<String> = vec![];
        for meeting_faculty in &self.meeting_faculty {
            let time = &meeting_faculty.meeting_time;
            let mut current_days: Vec<String> = vec![];
            if time.monday {
                days.push("MO".to_string());
                current_days.push("MO".to_string());
            }
            if time.tuesday {
                days.push("TU".to_string());
                current_days.push("TU".to_string());
            }
            if time.wednesday {
                days.push("WE".to_string());
                current_days.push("WE".to_string());
            }
            if time.thursday {
                days.push("TH".to_string());
                current_days.push("TH".to_string());
            }
            if time.friday {
                days.push("FR".to_string());
                current_days.push("FR".to_string());
            }
            day_lines.push(format!(
                "{} {}: {} - {}",
                "-".repeat(4),
                current_days.join(", "),
                time.start_time.clone().unwrap_or("None".to_string()),
                time.end_time.clone().unwrap_or("None".to_string())
            ))
        }
        write!(
            f,
            "{} ({} {}): {}\n{}",
            self.course_title,
            self.subject_course,
            self.sequence_number,
            days.join(", "),
            day_lines.join("\n")
        )
    }
}

fn find_schedules(course_subject_to_section: HashMap<String, Vec<&Section>>) {
    let needed_subject_codes: Vec<String> = vec!["ASST2012".to_string(), "REL3301".to_string()];

    for needed_subject_code in needed_subject_codes {
        let sections = course_subject_to_section.get(&needed_subject_code).unwrap();

        for section in sections {
            println!("{}", section)
        }
    }
}

fn main() {
    let file_path = PathBuf::from("japan").join("test.json");
    let file = File::open(file_path).expect("File did not open");
    let reader = BufReader::new(file);

    let sections: Vec<Section> = serde_json::from_reader(reader).unwrap();
    let mut course_subject_to_section: HashMap<String, Vec<&Section>> = HashMap::new();
    for section in sections.iter() {
        course_subject_to_section
            .entry(section.subject_course.clone())
            .or_insert(Vec::new())
            .push(section);
    }
    // find_schedules(course_subject_to_section);
    let title_keywords: Vec<String> = vec![
        // "literature".to_uppercase().to_string(),
        // "religion".to_uppercase().to_string(),
        "art".to_uppercase().to_string(),
    ];
    for section in sections.iter() {
        for keyword in title_keywords.iter() {
            if section.course_title.to_uppercase().contains(keyword) {
                println!("{}", section);
                break;
            }
        }
        // if section.credits == 2 {
        //     println!("{}", section)
        // }
    }
}
