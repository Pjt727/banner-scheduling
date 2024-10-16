#![allow(dead_code)]
use serde::Deserialize;
// use serde_json::value::Index;
// use serde_json::Result;
// use std::cmp::Ordering;
use std::collections::HashMap;
// use std::env;
use std::fs::File;
// use std::hash::{Hash, Hasher};
use std::io::BufReader;
use std::path::PathBuf;
mod data;
use data::{DaysChecked, Section, SectionCollection};
mod ui;

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
    let mut section_collection = SectionCollection::new(sections.iter().collect());
    section_collection.whitelist_credits(vec![2, 1]);
    section_collection.has_days();
    // section_collection.query("art");
    section_collection.whitelist_days(&DaysChecked {
        monday: false,
        tuesday: true,
        wednesday: false,
        thursday: true,
        friday: false,
        saturday: false,
        sunday: false,
    });
    for section in section_collection.running_sections.iter() {
        println!("{}", section)
    }
}
