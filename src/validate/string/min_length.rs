use super::{ValidateString, Validator};

#[derive(Clone, Copy)]
pub struct MinLengthValidator<'a> {
  pub message: Option<&'a str>,
  pub min_length: usize,
}

impl<'a> MinLengthValidator<'a> {
  pub fn new(min_length: usize, message: Option<&'a str>) -> MinLengthValidator {
    MinLengthValidator { 
      message, 
      min_length
    }
  }
}

impl<'a> Validator<'a> for MinLengthValidator<'a> {
  fn parse(&self, value: &'a str) -> Result<&'a str, String> {
    if value.len() < self.min_length  {
      Err(
        match self.message {
          Some(val) => val.into(),
          None => format!("min string length is {}", self.min_length)
        }
      )
    } else {
      Ok(value)
    }
  }
}

impl ValidateString {
  pub fn min(mut self, min_length: usize) -> Self {
    self.validators.push(Box::new(MinLengthValidator::new(min_length, None)));
    self
  }

  pub fn min_message(mut self, min_length: usize, message: &'static str) -> Self {
    self.validators.push(Box::new(MinLengthValidator::new(min_length, Some(message))));
    self
  }
}
