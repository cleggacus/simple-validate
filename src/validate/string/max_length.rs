use super::{ValidateString, Validator};

#[derive(Clone, Copy)]
pub struct MaxLengthValidator<'a> {
  pub message: Option<&'a str>,
  pub max_length: usize,
}

impl<'a> MaxLengthValidator<'a> {
  pub fn new(max_length: usize, message: Option<&'a str>) -> MaxLengthValidator {
    MaxLengthValidator { 
      message, 
      max_length 
    }
  }
}

impl<'a> Validator<'a> for MaxLengthValidator<'a> {
  fn parse(&self, value: &'a str) -> Result<&'a str, String> {
    if value.len() > self.max_length  {
      Err(
        match self.message {
          Some(val) => val.into(),
          None => format!("max string length is {}", self.max_length)
        }
      )
    } else {
      Ok(value)
    }
  }
}

impl ValidateString {
  pub fn max(mut self, max_length: usize) -> Self {
    self.validators.push(Box::new(MaxLengthValidator::new(max_length, None)));
    self
  }

  pub fn max_message(mut self, max_length: usize, message: &'static str) -> Self {
    self.validators.push(Box::new(MaxLengthValidator::new(max_length, Some(message))));
    self
  }
}