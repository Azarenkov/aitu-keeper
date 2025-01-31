use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ApiError {
    UserAlreadyExists,
    InvalidToken,
    
    UserNotFound,
    CoursesNotFound,
    GradesNotFound,
    DeadlinesNotFound,
    
    UserDataIsEmpty,
    CoursesAreEmpty,
    GradesAreEmpty,
    DeadlinesAreEmpty,
    
    UserAlreadyDeleted,
    
    InternalServerError,
}

impl ApiError {
    pub fn as_http_response(&self) -> HttpResponse {
        match self {
            ApiError::UserAlreadyExists => HttpResponse::Accepted().json(self.to_string()),
            ApiError::InvalidToken => HttpResponse::BadRequest().json(self.to_string()),
            ApiError::UserAlreadyDeleted => HttpResponse::Gone().json(self.to_string()),
            
            ApiError::UserNotFound
            | ApiError::CoursesNotFound
            | ApiError::GradesNotFound
            | ApiError::DeadlinesNotFound => HttpResponse::NotFound().json(self.to_string()),
            
            ApiError::UserDataIsEmpty
            | ApiError::CoursesAreEmpty
            | ApiError::GradesAreEmpty
            | ApiError::DeadlinesAreEmpty => HttpResponse::NoContent().json(self.to_string()),
            
            _ => HttpResponse::InternalServerError().json(ApiError::InternalServerError.to_string()),
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::UserAlreadyExists => write!(f, "User already exists"),
            ApiError::InvalidToken => write!(f, "Invalid token"),
            
            ApiError::UserNotFound=> write!(f, "User not found"),
            ApiError::CoursesNotFound => write!(f, "Courses not found"),
            ApiError::GradesNotFound => write!(f, "Grades not found"),
            ApiError::DeadlinesNotFound => write!(f, "Deadlines not found"),
            
            ApiError::UserDataIsEmpty=> write!(f, "User data is empty"),
            ApiError::CoursesAreEmpty => write!(f, "Courses are empty"),
            ApiError::GradesAreEmpty => write!(f, "Grades are empty"),
            ApiError::DeadlinesAreEmpty => write!(f, "Deadlines are empty"),

            ApiError::InternalServerError => write!(f, "Internal server error"),
            ApiError::UserAlreadyDeleted => write!{f, "User already deleted"}
        }
    }
}

impl std::error::Error for ApiError {}