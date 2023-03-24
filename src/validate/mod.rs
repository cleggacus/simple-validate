use self::string::ValidateString;

pub mod string;

pub struct Validate {}

impl<'a> Validate {
  pub fn string() -> ValidateString {
    ValidateString::new()
  }
}
