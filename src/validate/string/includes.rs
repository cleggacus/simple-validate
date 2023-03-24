use super::{ValidateString, Validator};

#[derive(Clone, Copy)]
pub struct Includes<'a> {
  pub message: Option<&'a str>,
  pub includes: &'a str,
}

impl<'a> Includes<'a> {
  pub fn new(includes: &'a str, message: Option<&'a str>) -> Includes<'a> {
    Includes { 
      message, 
      includes
    }
  }
}

impl<'a> Validator<'a> for Includes<'a> {
  fn parse(&self, value: &'a str) -> Result<&'a str, String> {
    if value.contains(self.includes) {
      Ok(value)
    } else {
      Err(
        match self.message {
          Some(val) => val.into(),
          None => format!("string must include {}", self.includes)
        }
      )
    }
  }
}

impl ValidateString {
  pub fn includes(mut self, includes: &'static str) -> Self {
    self.validators.push(Box::new(Includes::new(includes, None)));
    self
  }

  pub fn includes_message(mut self, includes: &'static str, message: &'static str) -> Self {
    self.validators.push(Box::new(Includes::new(includes, Some(message))));
    self
  }
}