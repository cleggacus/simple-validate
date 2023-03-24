use super::{ValidateString, Validator};

#[derive(Clone, Copy)]
pub struct EndsWith<'a> {
  pub message: Option<&'a str>,
  pub ends_with: &'a str,
}

impl<'a> EndsWith<'a> {
  pub fn new(ends_with: &'a str, message: Option<&'a str>) -> EndsWith<'a> {
    EndsWith { 
      message, 
      ends_with
    }
  }
}

impl<'a> Validator<'a> for EndsWith<'a> {
  fn parse(&self, value: &'a str) -> Result<&'a str, String> {
    if value.ends_with(self.ends_with) {
      Ok(value)
    } else {
      Err(
        match self.message {
          Some(val) => val.into(),
          None => format!("string must start with {}", self.ends_with)
        }
      )
    }
  }
}

impl ValidateString {
  pub fn ends_with(mut self, ends_with: &'static str) -> Self {
    self.validators.push(Box::new(EndsWith::new(ends_with, None)));
    self
  }

  pub fn ends_with_message(mut self, ends_with: &'static str, message: &'static str) -> Self {
    self.validators.push(Box::new(EndsWith::new(ends_with, Some(message))));
    self
  }
}