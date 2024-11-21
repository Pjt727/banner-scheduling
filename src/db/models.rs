use chrono::{DateTime, Utc};
use diesel::prelude::*;

pub const SPRING: &str = "Spring";
pub const SUMMER: &str = "Summer";
pub const WINTER: &str = "Winter";
pub const FALL: &str = "Fall";

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::terms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Term {
    pub id: String,
    pub season: String,
    pub year: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::faculty_members)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FacultyMember {
    pub id: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::meeting_times)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MeetingTime {
    pub id: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub meeting_type: Option<String>,
    pub start_minutes: Option<i32>,
    pub end_minutes: Option<i32>,
    pub is_monday: bool,
    pub is_tuesday: bool,
    pub is_wednesday: bool,
    pub is_thursday: bool,
    pub is_friday: bool,
    pub is_saturday: bool,
    pub is_sunday: bool,
    pub section_id: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::sections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Section {
    pub id: String,
    pub max_enrollment: Option<i32>,
    pub instruction_method: Option<String>,
    pub campus: Option<String>,
    pub enrollment: Option<i32>,
    pub course_id: String,
    pub primary_faculty_id: Option<String>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::courses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Course {
    pub id: String,
    pub title: Option<String>,
    pub credit_hours: i32,
    pub subject_code: Option<String>,
    pub number: Option<String>,
    pub subject_description: Option<String>,
    pub description: Option<String>,
}
