use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct Employee {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: String,
    #[validate(range(min = 18, max = 65, message = "Age must be between 18 and 65"))]
    pub age: u32,
    #[validate(length(min = 6, message = "Role must be at least 6 characters long"))]
    pub role: String,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
}
