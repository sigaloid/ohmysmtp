# OhMySMTP - Rust client

### Example usage:

#### Cargo.toml:

```toml
ohmysmtp = { git = "https://github.com/sigaloid/ohmysmtp" }
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

### Show appreciation

Want to say thanks for this library? Just click the button below and leave a brief note. It would make my day :)

[![Click me to show appreciation](https://img.shields.io/badge/Say%20Thanks-%F0%9F%A6%80%F0%9F%A6%80%F0%9F%A6%80-1EAEDB.svg)](https://saythanks.io/to/sigaloid)
