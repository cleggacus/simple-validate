<h1 align="center">
  Simple Validate
</h1>

<p align="center">
  Super simple way to validte inputs in rust
</p>


<p align="center">
  <a href="#">
    <img alt="Stars" src="https://img.shields.io/github/stars/cleggacus/simple-validate?color=yellow&style=for-the-badge">
  </a>
  &nbsp;
  <!-- <a href="#">
    <img alt="Fork" src="https://img.shields.io/github/forks/cleggacus/my-paste-bin?style=for-the-badge" />
  </a> -->
  &nbsp;
  <a href="https://github.com/cleggacus/my-paste-bin/issueshttps://trpc.io/discord">
    <img alt="Issues" src="https://img.shields.io/github/issues/cleggacus/simple-validate?color=red&style=for-the-badge" />
  </a>
  &nbsp;
  <a href="https://github.com/cleggacus/my-paste-bin/blob/main/LICENSE">
    <img alt="LICENSE" src="https://img.shields.io/github/license/cleggacus/simple-validate?label=license&style=for-the-badge" />
  </a>
</p>

## Getting Started

This crate is early in development and mainly a proof of concept but if you wish to try it out you can add the following line to your `Cargo.toml` file

```toml
simple-validate = { git = "https://github.com/cleggacus/simple-validate" }
```

## Credentials Example
### Code
```rs
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
```
### Output
```rs
Ok("domain@example.com")
Err(
    [
        "must contain an uppercase character",
        "must contain number or special character",
        "min string length is 8",
    ],
)
```
