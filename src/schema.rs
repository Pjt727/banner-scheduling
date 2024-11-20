use diesel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromSqlRow, AsExpression)]
#[sql_type = "Text"]
pub enum Season {
    Spring,
    Fall,
    Winter,
    Summer,
}

impl<DB: Backend> ToSql<Text, DB> for Season {
    fn to_sql<W: Output<DB>>(&self, out: &mut W) -> DieselResult {
        match *self {
            Season::Spring => out.write_all(b"Spring")?,
            Season::Fall => out.write_all(b"Fall")?,
            Season::Winter => out.write_all(b"Winter")?,
            Season::Summer => out.write_all(b"Summer")?,
        }
        Ok(IsNull::No)
    }
}

impl<DB: Backend> FromSql<Text, DB> for Season {
    fn from_sql(bytes: Option<&DB::RawValue>) -> DieselResult<Self> {
        match not_none!(bytes).as_ref() {
            b"Spring" => Ok(Season::Spring),
            b"Fall" => Ok(Season::Fall),
            b"Winter" => Ok(Season::Winter),
            b"Summer" => Ok(Season::Summer),
            _ => Err("Invalid season value".into()),
        }
    }
}

diesel::table! {
    use super::Season;
    use diesel::types::*;
    terms {
        id -> Text,
        /// Unqiue with year
        season -> Season,
        /// Unqiue with season
        year -> Integer,
    }
}

diesel::table! {
    faculty_members {
        id -> Text,
        /// Unqiue
        email -> Nullable<Text>,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
    }
}

diesel::table! {
    meeting_times {
        id -> Text,
        start_date -> Nullable<Text>,
        end_date -> Nullable<Text>,
        meeting_type -> Nullable<Text>,
        start_minutes -> Nullable<Integer>,
        end_minutes -> Nullable<Integer>,
        is_monday -> Bool,
        is_tuesday -> Bool,
        is_wednesday -> Bool,
        is_thursday -> Bool,
        is_friday -> Bool,
        is_saturday -> Bool,
        is_sunday -> Bool,
        section_id -> Text
    }
}

diesel::table! {
    sections {
        id -> Text,
        max_enrollment -> Nullable<Integer>,
        instruction_method -> Nullable<Text>,
        campus -> Nullable<Text>,
        enrollment -> Nullable<Integer>,
        course_id -> Text,
        primary_faculty_id -> Nullable<Text>,
    }
}

diesel::table! {
    courses {
        id -> Text,
        subject_code -> Nullable<Text>,
        number -> Nullable<Text>,
        subject_description -> Nullable<Text>,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        credit_hours -> Nullable<Integer>,
    }
}
