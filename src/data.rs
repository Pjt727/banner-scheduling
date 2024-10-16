use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serde::Deserialize;
use std::cmp::max;
use std::fmt;

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
pub struct Section {
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

pub struct DaysChecked {
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}

impl Section {
    fn in_days(&self, days: &DaysChecked) -> bool {
        // x -> y (implies) <==> !x || y
        // if there's a meeting time of that day it must be checked
        self.meeting_faculty.iter().all(|fac| {
            (!fac.meeting_time.monday || days.monday)
                && (!fac.meeting_time.tuesday || days.tuesday)
                && (!fac.meeting_time.wednesday || days.wednesday)
                && (!fac.meeting_time.thursday || days.thursday)
                && (!fac.meeting_time.friday || days.friday)
                && (!fac.meeting_time.saturday || days.saturday)
                && (!fac.meeting_time.sunday || days.sunday)
        })
    }

    fn has_set_days(&self) -> bool {
        self.meeting_faculty.iter().all(|fac| {
            (fac.meeting_time.monday)
                || (fac.meeting_time.tuesday)
                || (fac.meeting_time.wednesday)
                || (fac.meeting_time.thursday)
                || (fac.meeting_time.friday)
                || (fac.meeting_time.saturday)
                || (fac.meeting_time.sunday)
        })
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

// a destructive collection of section pointers
pub struct SectionCollection<'a> {
    // side affects on running_sections are used to
    //    produce the final result of the search
    pub running_sections: Vec<&'a Section>,
    matcher: SkimMatcherV2,
}

impl<'a> SectionCollection<'a> {
    pub fn new(sections: Vec<&'a Section>) -> Self {
        SectionCollection {
            running_sections: sections,
            matcher: SkimMatcherV2::default(),
        }
    }
    pub fn search(
        &mut self,
        search_term: Option<&str>,
        days_white_listed: Option<&DaysChecked>,
        credits_white_listed: Option<Vec<u8>>,
    ) {
        if let Some(days_white_listed) = days_white_listed {
            self.whitelist_days(days_white_listed);
        }
        if let Some(search_term) = search_term {
            self.query(search_term)
        }
        if let Some(credits_white_listed) = credits_white_listed {
            self.whitelist_credits(credits_white_listed)
        }
    }

    pub fn query(&mut self, search_term: &str) {
        let mut section_scores = vec![];
        for section in self.running_sections.iter() {
            let mut max_match = self
                .matcher
                .fuzzy_match(&section.course_title, search_term)
                .unwrap_or(i64::MIN);
            max_match = max(
                max_match,
                self.matcher
                    .fuzzy_match(&section.course_number, search_term)
                    .unwrap_or(i64::MIN),
            );
            max_match = max(
                max_match,
                self.matcher
                    .fuzzy_match(&section.subject, search_term)
                    .unwrap_or(i64::MIN),
            );
            max_match = max(
                max_match,
                self.matcher
                    .fuzzy_match(&section.subject_course, search_term)
                    .unwrap_or(i64::MIN),
            );
            if max_match != i64::MIN {
                section_scores.push((section, max_match));
            }
        }
        section_scores.sort_by(|(_, score1), (_, score2)| score2.cmp(score1));
        self.running_sections = section_scores.into_iter().map(|(sec, _)| *sec).collect()
    }

    pub fn whitelist_days(&mut self, days: &DaysChecked) {
        self.running_sections.retain(|s| s.in_days(days))
    }

    pub fn has_days(&mut self) {
        self.running_sections.retain(|s| s.has_set_days())
    }

    pub fn whitelist_credits(&mut self, possible_credits: Vec<u8>) {
        self.running_sections
            .retain(|s| possible_credits.contains(&s.credits))
    }
}
