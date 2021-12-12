use ohmysmtp::{Email, Error, File, FileType, OhMySmtp};

#[test]
fn test_str() {
    let email_service = OhMySmtp::new("API_KEY");

    let _result = email_service.send(&Email::new(
        "from@email.address",
        "to@email.address",
        "Body text",
    ));

    let email_advanced_example = Email::new("from@email.address", "to@email.address", "Body text")
        .with_subject("Subject line")
        .with_attachment(File::new(b"File!", "file-name.txt", &FileType::Txt));

    match email_service.send(&email_advanced_example) {
        Ok(()) => println!("Success!"),
        Err(_) => println!("Error :("),
    }
}

#[test]
fn test_string() {
    let email_service = OhMySmtp::new(format!("API_KEY"));

    let _result = email_service.send(&Email::new(
        format!("from@email.address"),
        format!("to@email.address"),
        format!("Body text"),
    ));

    let email_advanced_example = Email::new(
        format!("from@email.address"),
        format!("to@email.address"),
        format!("Body text"),
    )
        .with_subject(format!("Subject line"))
        .with_attachment(File::new(
            b"File!",
            format!("file-name.txt"),
            &FileType::Txt,
        ));

    match email_service.send(&email_advanced_example) {
        Ok(()) => println!("Success!"),
        Err(_) => println!("Error :("),
    }
}

#[test]
#[cfg(feature = "email-validation")]
fn test_invalid_email() {
    let email_service = OhMySmtp::new(format!("API_KEY"));

    assert_eq!(email_service.send(&Email::new(
        format!("from@email.address"),
        format!("test@-iana.org"),
        format!("Body text"),
    )), Err(Error::InvalidEmail));
}