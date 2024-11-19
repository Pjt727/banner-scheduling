DROP TABLE IF EXISTS Terms;
DROP TABLE IF EXISTS FacultyMembers;
DROP TABLE IF EXISTS MeetingTimes;
DROP TABLE IF EXISTS Sections;
DROP TABLE IF EXISTS SectionFacultyGroupers;
DROP TABLE IF EXISTS Courses;

CREATE TABLE Terms (
    id TEXT PRIMARY KEY,
    season TEXT CHECK (season IN ('Spring', 'Fall', 'Winter', 'Summer')),
    year INTEGER NOT NULL,
    UNIQUE(season, year)
);

CREATE TABLE FacultyMembers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email_address TEXT UNQIUE,
    first_name TEXT,
    last_name TEXT
);

CREATE TABLE MeetingTimes (
    id TEXT PRIMARY KEY,
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
    section_id TEXT,
    FOREIGN KEY (section_id) REFERENCES Section(id)
);


CREATE TABLE Sections (
    id TEXT PRIMARY KEY,
    max_enrollment INTEGER,
    instruction_method TEXT,
    campus TEXT,
    enrollment INTEGER,
    course_id TEXT,
    primary_faculty_id TEXT,
    FOREIGN KEY (course_id) REFERENCES Courses(id)
);


CREATE TABLE Courses (
    id TEXT PRIMARY KEY,
    subject_code TEXT,
    number TEXT,
    subject_description TEXT,
    title TEXT,
    description TEXT,
    credit_hours INTEGER
);
