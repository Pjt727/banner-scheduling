use serde::{de, Deserialize, Deserializer};

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
