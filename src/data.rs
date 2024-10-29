use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::cmp::max;
use std::fmt;

trait SectionLike {
    fn get_sequence_number(&self) -> String {}
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
impl PartialEq for DaysChecked {
    fn eq(&self, other: &Self) -> bool {
        self.monday == other.monday
            && self.tuesday == other.tuesday
            && self.wednesday == other.wednesday
            && self.thursday == other.thursday
            && self.friday == other.friday
            && self.saturday == other.saturday
            && self.sunday == other.sunday
    }
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
                time.start_time.clone().unwrap_or(0),
                time.end_time.clone().unwrap_or(0)
            ))
        }
        write!(
            f,
            "{} ({} {}): {} | {} Credits\n{}",
            self.course_title,
            self.subject_course,
            self.sequence_number,
            days.join(", "),
            self.credits,
            day_lines.join("\n")
        )
    }
}

// a destructive collection of section pointers
pub struct SectionCollection {
    sections: Vec<Section>,
    running_section_indexes: Vec<usize>,
    matcher: SkimMatcherV2,
    last_search_criteria: SearchCriteria,
}

#[derive(Default)]
pub struct SearchCriteria {
    pub query: Option<String>,
    pub whitelisted_days: Option<DaysChecked>,
    pub whitelisted_credits: Option<Vec<u8>>,
    pub whitelisted_campuses: Option<Vec<String>>,
    pub has_days: bool,
}

impl PartialEq for SearchCriteria {
    fn eq(&self, other: &Self) -> bool {
        self.query == other.query
            && self.whitelisted_days == other.whitelisted_days
            && self.whitelisted_credits == other.whitelisted_credits
            && self.whitelisted_campuses == other.whitelisted_campuses
            && self.has_days == other.has_days
    }
}

impl SectionCollection {
    pub fn new(sections: Vec<Section>) -> Self {
        SectionCollection {
            sections,
            running_section_indexes: vec![],
            matcher: SkimMatcherV2::default(),
            last_search_criteria: SearchCriteria {
                ..SearchCriteria::default()
            },
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
        self.running_section_indexes = (0..self.sections.len()).collect()
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

impl Default for SectionCollection {
    fn default() -> Self {
        return SectionCollection::new(vec![]);
    }
}
