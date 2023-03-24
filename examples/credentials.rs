use simple_validate::validate::Validate;

fn main() {
  let password_validator = Validate::string()
    .regex_message(
      r"(?=.*[a-z])", 
      "must contain a lowercase character"
    )
    .regex_message(
      r"(?=.*[A-Z])",
      "must contain an uppercase character"
    )
    .regex_message(
      r#"(?=.*[0-9\!@#$%^&*()\[\]{}\-_+=~`|:;"'<>,./?])"#,
      "must contain number or special character"
    )
    .min(8)
    .max(128);

  let email_validator = Validate::string().email();

  let email = "domain@example.com";
  let password = "pass";

  println!("{:?}", email_validator.parse(email));
  println!("{:#?}", password_validator.parse(password));
}