use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum RegistrationError {
    UserAlreadyExists,
    InvalidToken,
    
    UserNotFound,
    CoursesNotFound,
    GradesNotFound,
    GradesOverviewNotFound,
    DeadlinesNotFound,
    
    UserDataIsEmpty,
    CoursesAreEmpty,
    GradesAreEmpty,
    GradesOverviewIsEmpty,
    DeadlinesAreEmpty,
    
    InternalServerError,
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistrationError::UserAlreadyExists => write!(f, "User already exists"),
            RegistrationError::InvalidToken => write!(f, "Invalid token"),
            
            RegistrationError::UserNotFound=> write!(f, "User not found"),
            RegistrationError::CoursesNotFound => write!(f, "Courses not found"),
            RegistrationError::GradesNotFound => write!(f, "Grades not found"),
            RegistrationError::GradesOverviewNotFound => write!(f, "Grades overview not found"),
            RegistrationError::DeadlinesNotFound => write!(f, "Deadlines not found"),
            
            RegistrationError::UserDataIsEmpty=> write!(f, "User data is empty"),
            RegistrationError::CoursesAreEmpty => write!(f, "Courses are empty"),
            RegistrationError::GradesAreEmpty => write!(f, "Grades are empty"),
            RegistrationError::GradesOverviewIsEmpty => write!(f, "Grades overview is empty"),
            RegistrationError::DeadlinesAreEmpty => write!(f, "Deadlines are empty"),

            RegistrationError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

impl std::error::Error for RegistrationError {}