use crate::data;
use serde::{de, Deserialize, Deserializer};
use std::fs::File;
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
    campus: Option<String>,
    #[serde(rename = "endTime", deserialize_with = "from_string_time")]
    end_time: Option<u16>,
    #[serde(rename = "beginTime", deserialize_with = "from_string_time")]
    start_time: Option<u16>,
    #[serde(rename = "meetingType")]
    meeting_type: String,
}
impl MeetingTime {
    fn get_unique_string(&self) -> String {
        let mut unique_string = String::new();

        if self.monday {
            unique_string.push_str("M");
        }
        if self.tuesday {
            unique_string.push_str("T");
        }
        if self.wednesday {
            unique_string.push_str("W");
        }
        if self.thursday {
            unique_string.push_str("Th");
        }
        if self.friday {
            unique_string.push_str("F");
        }
        if self.saturday {
            unique_string.push_str("Sa");
        }
        if self.sunday {
            unique_string.push_str("Su");
        }

        if let Some(campus) = &self.campus {
            unique_string.push_str(campus);
        }

        if let Some(start_time) = self.start_time {
            unique_string.push_str(&start_time.to_string());
        }

        if let Some(end_time) = self.end_time {
            unique_string.push_str(&end_time.to_string());
        }

        unique_string.push_str(&self.meeting_type);

        unique_string
    }
}

fn from_string_time<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    let maybe_s: Option<String> = Deserialize::deserialize(deserializer)?;
    let s = match maybe_s {
        Some(s) => s,
        None => return Ok(None),
    };
    let hours: u16 = s[..2]
        .parse()
        .map_err(|_| de::Error::custom("Could not parse hours"))?;
    let minutes: u16 = s[2..4]
        .parse()
        .map_err(|_| de::Error::custom("Could not parse minutes"))?;
    return Ok(Some(hours * 60 + minutes));
}

#[derive(Deserialize, Debug)]
struct MeetingsFaculty {
    #[serde(rename = "meetingTime")]
    meeting_time: MeetingTime,
}

#[derive(Deserialize, Debug)]
struct Faculty {
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "emailAddress")]
    email_address: String,
    #[serde(rename = "primaryIndicator")]
    primary_indicator: bool,
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
    seats_available: u16,
    #[serde(rename = "maximumEnrollment")]
    maximum_enrollment: u16,
    #[serde(rename = "instructionalMethod")]
    instructional_method: String,
    #[serde(rename = "openSection")]
    open_section: bool,
    #[serde(rename = "meetingsFaculty")]
    meeting_faculty: Vec<MeetingsFaculty>,
    #[serde(rename = "faculty")]
    faculty: Vec<Faculty>,
    #[serde(rename = "creditHourLow")]
    credits: u8,
    #[serde(rename = "subjectCourse")]
    subject_course: String,
    #[serde(rename = "subjectDescription")]
    subject_description: String,
    #[serde(rename = "campusDescription")]
    campus_description: String,
}

impl Section {
    fn get_course_id(&self) -> String {
        return format!("{},{}", self.subject, self.course_number);
    }

    fn get_section_id(&self) -> String {
        return format!("{},{}", self.get_course_id(), self.sequence_number);
    }
}

fn into_data_section(section: &Section) -> data::Section {
    data::Section {
        id: section.get_section_id(),
        course_id: section.get_course_id(),
        max_enrollment: Some(section.maximum_enrollment),
        instruction_method: Some(section.instructional_method.clone()),
        campus: Some(section.campus_description.clone()),
        enrollment: Some(section.maximum_enrollment),
        primary_faculty_id: section
            .faculty
            .iter()
            .filter(|f| f.primary_indicator)
            .next()
            .map(|f| f.email_address.clone()),
    }
}

fn into_data_course(section: &Section) -> data::Course {
    data::Course {
        id: section.get_course_id(),
        title: section.course_title.clone(),
        credit_hours: section.credits.clone(),
        subject_code: Some(section.subject.clone()),
        number: Some(section.course_number.clone()),
        subject_description: Some(section.subject_description.clone()),
        description: None,
    }
}

fn into_faculty(section: &Section) -> Vec<data::FacultyMember> {
    section
        .faculty
        .iter()
        .map(|s| data::FacultyMember {
            id: s.email_address.clone(),
            name: s.display_name.clone(),
            email_address: Some(s.email_address.clone()),
            first_name: s.display_name.split(' ').next().map(|f| f.to_string()),
            last_name: s.display_name.split(' ').last().map(|f| f.to_string()),
        })
        .collect()
}

fn into_meeting_times(section: &Section) -> Vec<data::MeetingTime> {
    section
        .meeting_faculty
        .iter()
        .map(|m_f| {
            let time = &m_f.meeting_time;
            data::MeetingTime {
                id: format!("{},{}", section.get_section_id(), time.get_unique_string()),
                days_checked: data::DaysChecked {
                    monday: time.monday,
                    tuesday: time.tuesday,
                    wednesday: time.wednesday,
                    thursday: time.thursday,
                    friday: time.friday,
                    saturday: time.saturday,
                    sunday: time.sunday,
                },
                start_date: None,
                end_date: None,
                start_minutes: time.start_time,
                end_minutes: time.end_time,
                meeting_type: Some(time.meeting_type.clone()),
                section_id: section.get_section_id(),
            }
        })
        .collect()
}

fn adapt_banner() -> (
    Vec<data::Course>,
    Vec<data::Section>,
    Vec<data::MeetingTime>,
    Vec<data::FacultyMember>,
) {
    let file_path = PathBuf::from("japan").join("test.json");
    let file = File::open(file_path).expect("File did not open");
    let reader = BufReader::new(file);
    let sections: Vec<Section> = serde_json::from_reader(reader).unwrap();

    (
        sections.iter().map(into_data_course).collect(),
        sections.iter().map(into_data_section).collect(),
        sections.iter().map(into_meeting_times).flatten().collect(),
        sections.iter().map(into_faculty).flatten().collect(),
    )
}
