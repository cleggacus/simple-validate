mod max_length;
mod min_length;
mod length;
mod regex;
mod email;

pub trait Validator<'a> {
  fn parse(&self, value: &'a str) -> Result<&'a str, String>;
}

// #[derive(Clone)]
pub struct ValidateString {
  pub validators: Vec<Box<dyn Validator<'static>>>
}

impl ValidateString {
  pub fn new() -> ValidateString {
    ValidateString {
      validators: Vec::new(),
    }
  }

  pub fn parse(&self, value: &'static str) -> Result<&'static str, Vec<String>> {
    let mut errors: Vec<String> = Vec::new();

    for validator in &self.validators {
      if let Err(err) = validator.parse(value) {
        errors.push(err);
      }
    }

    if !errors.is_empty() {
      return Err(errors);
    }

    Ok(value)
  }
}

