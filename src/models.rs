use crate::schema::Season;
use diesel::backend::Backend;
use diesel::deserialize::{FromSql, Result as DieselResult};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::{Integer, Text};
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromSqlRow, AsExpression, Queryable, Selectable)]
#[diesel(table_name = crate::schema::terms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Term {
    pub season: Season,
    pub year: i32,
}

pub struct FacultyMember {
    pub id: String,
    pub name: String,
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub struct MeetingTime {
    pub id: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub meeting_type: Option<String>,
    pub start_minutes: Option<u16>,
    pub end_minutes: Option<u16>,
    pub days_checked: DaysChecked,
    pub section_id: String,
}

pub struct Section {
    pub id: String,
    pub max_enrollment: Option<u16>,
    pub instruction_method: Option<String>,
    pub campus: Option<String>,
    pub enrollment: Option<u16>,
    pub course_id: String,
    pub primary_faculty_id: Option<String>,
}

pub struct Course {
    pub id: String,
    pub title: String,
    pub credit_hours: u8,
    pub subject_code: Option<String>,
    pub number: Option<String>,
    pub subject_description: Option<String>,
    pub description: Option<String>,
}
