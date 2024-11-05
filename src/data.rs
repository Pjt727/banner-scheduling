use chrono::{DateTime, Utc};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::cmp::max;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

enum Season {
    Spring,
    Fall,
    Winter,
    Summer,
}

struct Term {
    season: Season,
    year: u16,
}

struct FacultyMember {
    name: String,
    email_address: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    is_primary: Option<bool>,
}

struct MeetingTime {
    days_checked: DaysChecked,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    start_minutes: Option<u16>,
    end_minutes: Option<u16>,
    meeting_type: Option<String>,
}

struct Section {
    max_enrollement: Option<u16>,
    instruction_method: Option<String>,
    campus: Option<String>,
    enrollement: Option<u16>,
    faculty: Vec<Rc<FacultyMember>>,
    meeting_times: Vec<MeetingTime>,
    course: Rc<Course>,
}

struct Course {
    subject_code: Option<String>,
    number: Option<String>,
    subject_description: Option<String>,
    title: String,
    description: Option<String>,
    credit_hours: u8,
    sections: Vec<Rc<Section>>,
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

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!();
    }
}

// a destructive collection of section pointers
pub struct TermCollection {
    sections: Vec<Section>,
    courses: Vec<Course>,
    faculty: Vec<FacultyMember>,
    running_courses: Vec<Rc<Course>>,
    matcher: SkimMatcherV2,
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
    pub fn new(sections: Vec<Section>, courses: Vec<Course>, faculty: Vec<FacultyMember>) -> Self {
        TermCollection {
            sections,
            faculty,
            running_courses: courses.iter().map(|course| Rc::clone(&course)).collect(),
            courses,
            matcher: SkimMatcherV2::default(),
            last_search_criteria: SearchCriteria::default(),
        }
    }

    pub fn search(&mut self, search_criteria: SearchCriteria) {
        if self.last_search_criteria == search_criteria {
            return;
        }
        self.reset();
        if let Some(query) = &search_criteria.query {
            self.query(query);
        }
        if let Some(whitelisted_days) = &search_criteria.whitelisted_days {
            self.whitelist_days(whitelisted_days);
        }
        if let Some(whitelisted_credits) = &search_criteria.whitelisted_credits {
            self.whitelist_credits(whitelisted_credits);
        }
        if let Some(whitelisted_campuses) = &search_criteria.whitelisted_campuses {
            self.whilelist_meeting_campuses(whitelisted_campuses);
        }
        if search_criteria.has_days {
            self.has_days();
        }

        self.last_search_criteria = search_criteria;
    }

    // could implement a more sophiscated reset that only resets
    //   what is needed
    #[inline]
    fn reset(&mut self) {
        self.running_courses = self.courses.iter().map(|s| Rc::new(*s)).collect()
    }

    pub fn get_running_sections(&self) -> impl Iterator<Item = &Section> {
        self.running_section_indexes
            .iter()
            .map(move |&index| &self.sections[index])
    }

    fn query(&mut self, search_term: &str) {
        let get_value = |i: &usize| {
            let section = &self.sections[*i];
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
            return max_match;
        };
        let mut indexes_scores = self
            .running_section_indexes
            .iter()
            .filter_map(|index| {
                let score = get_value(index);
                if score == i64::MIN {
                    None
                } else {
                    Some((index, score))
                }
            })
            .collect::<Vec<(&usize, i64)>>();
        indexes_scores.sort_by(|(_, score1), (_, score2)| score1.cmp(score2));
        self.running_section_indexes = indexes_scores.into_iter().map(|(sec, _)| *sec).collect()
    }

    fn whitelist_days(&mut self, days: &DaysChecked) {
        self.running_section_indexes
            .retain(|i| self.sections[*i].in_days(days))
    }

    fn has_days(&mut self) {
        self.running_section_indexes
            .retain(|i| self.sections[*i].has_set_days())
    }

    fn whitelist_credits(&mut self, possible_credits: &Vec<u8>) {
        self.running_section_indexes
            .retain(|i| possible_credits.contains(&self.sections[*i].credits))
    }

    fn whilelist_meeting_campuses(&mut self, campuses: &Vec<String>) {
        self.running_section_indexes.retain(|i| {
            self.sections[*i].meeting_faculty.iter().all(|m| {
                if let Some(campus) = &m.meeting_time.campus {
                    campuses.contains(campus)
                } else {
                    true
                }
            })
        })
    }

    fn whitelist_subject_courses(&mut self, subject_courses: &Vec<&str>) {
        self.running_section_indexes
            .retain(|i| subject_courses.contains(&&self.sections[*i].subject_course.as_str()))
    }

    fn rough_time_sort(&mut self) {
        self.running_section_indexes.sort_by(|i1, i2| {
            self.sections[*i1]
                .meeting_faculty
                .first()
                .unwrap()
                .meeting_time
                .start_time
                .cmp(
                    &self.sections[*i2]
                        .meeting_faculty
                        .first()
                        .unwrap()
                        .meeting_time
                        .start_time,
                )
        })
    }

    fn rough_whitelist_start_time(&mut self, start_time: u16) {
        self.running_section_indexes.retain(|i| {
            if let Some(fac) = self.sections[*i].meeting_faculty.first() {
                if let Some(start) = &fac.meeting_time.start_time {
                    return *start == start_time;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        })
    }
}

impl Default for TermCollection {
    fn default() -> Self {
        return TermCollection::new(vec![]);
    }
}
