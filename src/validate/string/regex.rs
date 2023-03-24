use fancy_regex::Regex;

use super::{ValidateString, Validator};

#[derive(Clone, Copy)]
pub struct RegexValidator<'a> {
  pub message: Option<&'a str>,
  pub regex: &'a str,
}

impl<'a> RegexValidator<'a> {
  pub fn new(regex: &'a str, message: Option<&'a str>) -> RegexValidator<'a> {
    RegexValidator {
      message, 
      regex, 
    }
  }

  pub fn parse(&self, value: &'a str) -> Result<&'a str, String> {
    let re = Regex::new(self.regex);
    
    match re {
      Ok(re) => {
        let result = re.is_match(value);

        match result {
          Ok(val) => {
            if val {
              Ok(value)
            } else {
              Err(
                match self.message {
                  Some(err) => err.into(),
                  None => String::from("Value did not match regex")
                }
              )
            }
          },
          Err(_) => {
            println!("1");
            Err("Could not parse regex".into())
          }
        }
      },
      Err(_) => {
        println!("2");
        Err("Could not parse regex".into())
      }
    }
  }
}

impl<'a> Validator<'a> for RegexValidator<'a> {
  fn parse(&self, value: &'a str) -> Result<&'a str, String> {
    let re = Regex::new(self.regex);
    
    match re {
      Ok(re) => {
        let result = re.is_match(value);

        match result {
          Ok(val) => {
            if val {
              Ok(value)
            } else {
              Err(
                match self.message {
                  Some(err) => err.into(),
                  None => String::from("Value did not match regex")
                }
              )
            }
          },
          Err(_) => {
            println!("1");
            Err("Could not parse regex".into())
          }
        }
      },
      Err(err) => {
        println!("2");
        Err(err.to_string())
      }
    }
  }
}

impl ValidateString {
  pub fn regex(mut self, regex: &'static str) -> Self {
    self.validators.push(Box::new(RegexValidator::new(regex, None)));
    self
  }

  pub fn regex_message(mut self, regex: &'static str, message: &'static str) -> Self {
    self.validators.push(Box::new(RegexValidator::new(regex, Some(message))));
    self
  }
}