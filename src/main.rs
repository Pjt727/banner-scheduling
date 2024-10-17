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
    /*
                        3 credits:
                lit JPNS 2012* | JPNS 2111
                eth REL 3301* | POLS 2496 <-- may not be approved
                req JPNS 2000

            BIOL0848
            CHEM0821
            ADV0853
            EDUC0823
            SOC0831
            SOC2130 (risk society / japanese education)


                        4 credits:
                        CIS 2166

                        2 credits:
                        ACTV1018 | ACTV1013 | DANC1807
        1 credits:
        STHM1115

    REL 3301
    EDUC 0823 | ASST2373
    JPNS 2111
    JPNS 2000
    SOC 2130

                             */

    let subject_codes = vec![
        "JPNS2012", "JPNS2111", "REL3301", "JPNS2000", "CHEM0821", "ADV0821", "EDUC0823",
        "SOC0831", "SOC2130",
    ];
    // let subject_codes = vec!["ACTV1018", "ACTV1013", "DANC1807"];

    let sections: Vec<Section> = serde_json::from_reader(reader).unwrap();
    let mut section_collection = SectionCollection::new(sections.iter().collect());
    section_collection.whitelist_credits(vec![3]);
    section_collection.has_days();
    section_collection.whitelist_subject_courses(subject_codes);
    // section_collection.query("justice");
    // section_collection.while_list_meeting_campus("ONL");
    // section_collection.while_list_meeting_campus("JPN");
    section_collection.whitelist_days(&DaysChecked {
        monday: false,
        tuesday: true,
        wednesday: false,
        thursday: true,
        friday: false,
        saturday: false,
        sunday: false,
    });
    section_collection.rough_time_sort();
    // section_collection.rough_whitelist_start_time("1020");
    for section in section_collection.running_sections.iter() {
        println!("{}", section)
    }
}
