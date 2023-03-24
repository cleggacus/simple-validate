use super::{ValidateString, Validator};

#[derive(Clone, Copy)]
pub struct StartsWith<'a> {
  pub message: Option<&'a str>,
  pub starts_with: &'a str,
}

impl<'a> StartsWith<'a> {
  pub fn new(starts_with: &'a str, message: Option<&'a str>) -> StartsWith<'a> {
    StartsWith { 
      message, 
      starts_with
    }
  }
}

impl<'a> Validator<'a> for StartsWith<'a> {
  fn parse(&self, value: &'a str) -> Result<&'a str, String> {
    if value.starts_with(self.starts_with) {
      Ok(value)
    } else {
      Err(
        match self.message {
          Some(val) => val.into(),
          None => format!("string must start with {}", self.starts_with)
        }
      )
    }
  }
}

impl ValidateString {
  pub fn starts_with(mut self, starts_with: &'static str) -> Self {
    self.validators.push(Box::new(StartsWith::new(starts_with, None)));
    self
  }

  pub fn starts_with_message(mut self, starts_with: &'static str, message: &'static str) -> Self {
    self.validators.push(Box::new(StartsWith::new(starts_with, Some(message))));
    self
  }
}