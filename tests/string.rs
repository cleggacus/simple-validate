#[cfg(test)]
mod string_tests {
  use simple_validate::validate::Validate;

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
    let result_ok = validator.parse("test@example.co.uk");

    assert!(result_err_1.is_err(), "email err");
    assert!(result_err_2.is_err(), "email err");
    assert!(result_ok.is_ok(), "email ok");
  }

  #[test]
  fn starts_with() {
    let validator = Validate::string().starts_with("hello");

    let result_err = validator.parse("world");
    let result_ok = validator.parse("hello world");

    assert!(result_err.is_err(), "starts_with err");
    assert!(result_ok.is_ok(), "starts_with ok");
  }

  #[test]
  fn ends_with() {
    let validator = Validate::string().ends_with("world");

    let result_err = validator.parse("hello");
    let result_ok = validator.parse("hello world");

    assert!(result_err.is_err(), "ends_with err");
    assert!(result_ok.is_ok(), "ends_with ok");
  }

  #[test]
  fn includes() {
    let validator = Validate::string().includes("there");

    let result_err = validator.parse("hello world");
    let result_ok = validator.parse("hello there world");

    assert!(result_err.is_err(), "includes_with err");
    assert!(result_ok.is_ok(), "includes_with ok");
  }
}
