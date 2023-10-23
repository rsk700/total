pub mod json;
pub mod ldm;

pub type VerificationResult = Result<(), Vec<String>>;

pub fn format_error(path: &[&str], message: &str) -> String {
    if path.is_empty() {
        message.to_owned()
    } else {
        format!("{}: {}", path.join("."), message)
    }
}

pub trait Verify {
    fn verify(&self, up_to_first_error: bool) -> VerificationResult;
}
