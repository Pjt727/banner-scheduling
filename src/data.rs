use chrono::{DateTime, Utc};
use sqlx;
use sqlx::{query, SqlitePool};
use std::cmp::max;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub async fn bulk_insert(
    pool: &SqlitePool,
    courses: Vec<Course>,
    sections: Vec<Section>,
    meeting_times: Vec<MeetingTime>,
    faculty_members: Vec<FacultyMember>,
) {
    todo!()
}

#[derive(Eq, PartialEq)]
pub struct DaysChecked {
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}

impl DaysChecked {
    fn has_set_days(&self) -> bool {
        self.monday
            || self.tuesday
            || self.wednesday
            || self.thursday
            || self.friday
            || self.saturday
            || self.sunday
    }
}

// a destructive collection of section pointers
pub struct TermCollection {
    last_search_criteria: SearchCriteria,
}

#[derive(Default, PartialEq, Eq)]
pub struct SearchCriteria {
    pub query: Option<String>,
    pub whitelisted_days: Option<DaysChecked>,
    pub whitelisted_credits: Option<Vec<u8>>,
    pub whitelisted_campuses: Option<Vec<String>>,
    pub has_days: bool,
}

impl TermCollection {
    pub fn new() -> Self {
        TermCollection {
            last_search_criteria: SearchCriteria::default(),
        }
    }

    pub fn search_courses(&mut self, pool: &SqlitePool, search_criteria: SearchCriteria) {
        if self.last_search_criteria == search_criteria {
            return;
        }
        if let Some(query) = &search_criteria.query {
            todo!();
        }
        if let Some(whitelisted_days) = &search_criteria.whitelisted_days {
            todo!();
        }
        if let Some(whitelisted_credits) = &search_criteria.whitelisted_credits {
            todo!();
        }
        if let Some(whitelisted_campuses) = &search_criteria.whitelisted_campuses {
            todo!();
        }
        if search_criteria.has_days {
            todo!();
        }

        self.last_search_criteria = search_criteria;
    }
    //
    // pub fn get_running_sections(&self) -> impl Iterator<Item = &Section> {
    //     self.running_section_indexes
    //         .iter()
    //         .map(move |&index| &self.sections[index])
    // }
    //
    // fn query(&mut self, search_term: &str) {
    //     let get_value = |i: &usize| {
    //         let section = &self.sections[*i];
    //         let mut max_match = self
    //             .matcher
    //             .fuzzy_match(&section.course_title, search_term)
    //             .unwrap_or(i64::MIN);
    //         max_match = max(
    //             max_match,
    //             self.matcher
    //                 .fuzzy_match(&section.course_number, search_term)
    //                 .unwrap_or(i64::MIN),
    //         );
    //         max_match = max(
    //             max_match,
    //             self.matcher
    //                 .fuzzy_match(&section.subject, search_term)
    //                 .unwrap_or(i64::MIN),
    //         );
    //         max_match = max(
    //             max_match,
    //             self.matcher
    //                 .fuzzy_match(&section.subject_course, search_term)
    //                 .unwrap_or(i64::MIN),
    //         );
    //         return max_match;
    //     };
    //     let mut indexes_scores = self
    //         .running_section_indexes
    //         .iter()
    //         .filter_map(|index| {
    //             let score = get_value(index);
    //             if score == i64::MIN {
    //                 None
    //             } else {
    //                 Some((index, score))
    //             }
    //         })
    //         .collect::<Vec<(&usize, i64)>>();
    //     indexes_scores.sort_by(|(_, score1), (_, score2)| score1.cmp(score2));
    //     self.running_section_indexes = indexes_scores.into_iter().map(|(sec, _)| *sec).collect()
    // }
    //
    // fn whitelist_days(&mut self, days: &DaysChecked) {
    //     self.running_section_indexes
    //         .retain(|i| self.sections[*i].in_days(days))
    // }
    //
    // fn has_days(&mut self) {
    //     self.running_section_indexes
    //         .retain(|i| self.sections[*i].has_set_days())
    // }
    //
    // fn whitelist_credits(&mut self, possible_credits: &Vec<u8>) {
    //     self.running_section_indexes
    //         .retain(|i| possible_credits.contains(&self.sections[*i].credits))
    // }
    //
    // fn whilelist_meeting_campuses(&mut self, campuses: &Vec<String>) {
    //     self.running_section_indexes.retain(|i| {
    //         self.sections[*i].meeting_faculty.iter().all(|m| {
    //             if let Some(campus) = &m.meeting_time.campus {
    //                 campuses.contains(campus)
    //             } else {
    //                 true
    //             }
    //         })
    //     })
    // }
    //
    // fn whitelist_subject_courses(&mut self, subject_courses: &Vec<&str>) {
    //     self.running_section_indexes
    //         .retain(|i| subject_courses.contains(&&self.sections[*i].subject_course.as_str()))
    // }
    //
    // fn rough_time_sort(&mut self) {
    //     self.running_section_indexes.sort_by(|i1, i2| {
    //         self.sections[*i1]
    //             .meeting_faculty
    //             .first()
    //             .unwrap()
    //             .meeting_time
    //             .start_time
    //             .cmp(
    //                 &self.sections[*i2]
    //                     .meeting_faculty
    //                     .first()
    //                     .unwrap()
    //                     .meeting_time
    //                     .start_time,
    //             )
    //     })
    // }
    //
    // fn rough_whitelist_start_time(&mut self, start_time: u16) {
    //     self.running_section_indexes.retain(|i| {
    //         if let Some(fac) = self.sections[*i].meeting_faculty.first() {
    //             if let Some(start) = &fac.meeting_time.start_time {
    //                 return *start == start_time;
    //             } else {
    //                 return false;
    //             }
    //         } else {
    //             return false;
    //         }
    //     })
    // }
}

impl Default for TermCollection {
    fn default() -> Self {
        return TermCollection::new();
    }
}
