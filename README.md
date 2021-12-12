# OhMySMTP - Rust client

[![Documentation](https://docs.rs/ohmysmtp/badge.svg)](https://docs.rs/ohmysmtp)
[![Latest version](https://img.shields.io/crates/v/ohmysmtp.svg)](https://crates.io/crates/ohmysmtp)

### Example usage:

#### Cargo.toml:

```toml
ohmysmtp = "0.1.0"
```

#### Code:

```rust
use ohmysmtp::{Email, File, FileType, OhMySmtp};

let email_service = OhMySmtp::new("API_KEY");

let result = email_service.send(&Email::new(
    "from@email.address",
    "to@email.address",
    "Body text",
    ));

    let email_advanced_example =
Email::new("from@email.address", "to@email.address", "Body text")
.with_subject("Subject line")
.with_attachment(File::new(b"File!", "file-name.txt", & FileType::Txt));

match email_service.send( & email_advanced_example) {
Ok(()) => println ! ("Success!"),
Err(e) => println ! ("Error :(")
}
```

### Roadmap

- [x] Add email validation with `email-address-parser`
- [ ] Add deliverability check with `check-if-email-exists`
- [ ] Maybe add temp email check (though I am sort of opposed to this as someone who uses them for crappy services :p)

### Changelog

#### 0.1.0

Initial release

#### 0.1.1

* Add optional local email validation before sending to OhMySmtp (behind `email-validation` feature)
* Reuse ureq agent (header reuse pending https://github.com/algesten/ureq/issues/444)

### Show appreciation

Want to say thanks for this library? Just click the button below and leave a brief note. It would make my day :)

[![Click me to show appreciation](https://img.shields.io/badge/Say%20Thanks-%F0%9F%A6%80%F0%9F%A6%80%F0%9F%A6%80-1EAEDB.svg)](https://saythanks.io/to/sigaloid)
