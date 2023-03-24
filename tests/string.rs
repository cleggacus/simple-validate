#[cfg(test)]
mod string_tests {
  use validator::validate::Validate;

  #[test]
  fn min() {
    let validator = Validate::string().min(10);

    let result_err = validator.parse("Hello");
    let result_ok = validator.parse("Hello World");

    assert!(result_err.is_err(), "min err");
    assert!(result_ok.is_ok(), "min ok");
  }

  #[test]
  fn max() {
    let validator = Validate::string().max(10);

    let result_err = validator.parse("Hello World");
    let result_ok = validator.parse("Hello");

    assert!(result_err.is_err(), "max err");
    assert!(result_ok.is_ok(), "max ok");
  }

  #[test]
  fn length() {
    let validator = Validate::string().length(11);

    let result_err = validator.parse("Hello");
    let result_ok = validator.parse("Hello World");

    assert!(result_err.is_err(), "length err");
    assert!(result_ok.is_ok(), "length ok");
  }

  #[test]
  fn regex() {
    let validator = Validate::string().regex(r"(?=.*[a-z])");

    let result_err = validator.parse("HELLO");
    let result_ok = validator.parse("hello");

    assert!(result_err.is_err(), "regex err");
    assert!(result_ok.is_ok(), "regex ok");

  }

  #[test]
  fn email() {
    let validator = Validate::string().email();

    let result_err_1 = validator.parse("test@examplecouk");
    let result_err_2 = validator.parse("testexample.co.uk");
    let result_err_3 = validator.parse("test@example.co.u@k");
    let result_ok = validator.parse("test@example.co.uk");

    assert!(result_err_1.is_err(), "email err");
    assert!(result_err_2.is_err(), "email err");
    assert!(result_ok.is_ok(), "email ok");
  }
}
