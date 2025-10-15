#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    /// The person's name
    pub name: String,
    /// The person's age in years
    pub age: u32,
}
/// Validate age is in reasonable range
fn validate_age(age: u32) -> bool {
    age > 0 && age < 150
}
/// Helper function manually added
fn helper_validate_name(name: &str) -> bool {
    !name.is_empty() && name.len() < 100
}
