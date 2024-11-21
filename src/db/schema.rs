use diesel;

diesel::table! {
    terms {
        id -> Text,
        /// Unqiue with year, is an enum (could possibly change)
        season -> Text,
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
        start_date -> Nullable<TimestamptzSqlite>,
        end_date -> Nullable<TimestamptzSqlite>,
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
        credit_hours -> Integer,
    }
}
