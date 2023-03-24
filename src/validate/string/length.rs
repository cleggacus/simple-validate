use super::{ValidateString, Validator};

#[derive(Clone, Copy)]
pub struct LengthValidator<'a> {
  pub message: Option<&'a str>,
  pub length: usize,
}

impl<'a> LengthValidator<'a> {
  pub fn new(length: usize, message: Option<&'a str>) -> LengthValidator {
    LengthValidator { 
      message, 
      length 
    }
  }
}

impl<'a> Validator<'a> for LengthValidator<'a> {
  fn parse(&self, value: &'a str) -> Result<&'a str, String> {
    if value.len() != self.length {
      Err(
        match self.message {
          Some(val) => val.into(),
          None => format!("string length must be {}", self.length)
        }
      )
    } else {
      Ok(value)
    }
  }
}

impl ValidateString {
  pub fn length(mut self, length: usize) -> Self {
    self.validators.push(Box::new(LengthValidator::new(length, None)));
    self
  }

  pub fn length_message(mut self, length: usize, message: &'static str) -> Self {
    self.validators.push(Box::new(LengthValidator::new(length, Some(message))));
    self
  }
}