CREATE TABLE Term (
    id INTEGER PRIMARY KEY,
    season TEXT CHECK (season IN ('Spring', 'Fall', 'Winter', 'Summer')),
    year INTEGER NOT NULL
);

CREATE TABLE FacultyMember (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    email_address TEXT,
    first_name TEXT,
    last_name TEXT,
    is_primary INTEGER NOT NULL
);

CREATE TABLE MeetingTime (
    id INTEGER PRIMARY KEY,
    start_date TEXT,
    end_date TEXT,
    meeting_type TEXT,
    start_minutes INTEGER,
    end_minutes INTEGER,
    is_monday INTEGER NOT NULL,
    is_tuesday INTEGER NOT NULL,
    is_wednesday INTEGER NOT NULL,
    is_thursday INTEGER NOT NULL,
    is_friday INTEGER NOT NULL,
    is_saturday INTEGER NOT NULL,
    is_sunday INTEGER NOT NULL,
    section_id INTEGER,
    FOREIGN KEY (section_id) REFERENCES Section(id)
);


CREATE TABLE Section (
    id INTEGER PRIMARY KEY,
    max_enrollment INTEGER,
    instruction_method TEXT,
    campus TEXT,
    enrollment INTEGER,
    course_id INTEGER,
    FOREIGN KEY (course_id) REFERENCES Course(id)
);

CREATE TABLE SectionFacultyGrouper (
    id INTEGER PRIMARY KEY,
    section_id INTEGER,
    faculty_id INTEGER,
    FOREIGN KEY (section_id) REFERENCES Course(id),
    FOREIGN KEY (faculty_id) REFERENCES FacultyMember(id)
);

CREATE TABLE Course (
    id INTEGER PRIMARY KEY,
    subject_code TEXT,
    number TEXT,
    subject_description TEXT,
    title TEXT,
    description TEXT,
    credit_hours INTEGER
);
