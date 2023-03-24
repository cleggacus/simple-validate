use super::{ValidateString, Validator, regex::RegexValidator};

#[derive(Clone, Copy)]
pub struct EmailValidator<'a> {
  pub message: Option<&'a str>,
}

impl<'a> EmailValidator<'a> {
  pub fn new(message: Option<&'a str>) -> EmailValidator {
    EmailValidator { 
      message, 
    }
  }
}

impl<'a> Validator<'a> for EmailValidator<'a> {
  fn parse(&self, value: &'a str) -> Result<&'a str, String> {
    let message = match self.message {
      Some(message) => message,
      None => "string must be an email"
    };

    RegexValidator::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,4}", Some(message))
      .parse(value)
  }
}

impl ValidateString {
  pub fn email(mut self) -> Self {
    self.validators.push(Box::new(EmailValidator::new(None)));
    self
  }

  pub fn email_message(mut self, message: &'static str) -> Self {
    self.validators.push(Box::new(EmailValidator::new(Some(message))));
    self
  }
}