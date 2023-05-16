use ohmysmtp::{Email, File, FileType};

// #[test]
// fn test_str() {
//     let email_service = OhMySmtp::new("API_KEY");

//     let _result = email_service.send(&Email::new(
//         "from@email.address",
//         "to@email.address",
//         "Body text",
//     ));

//     let email_advanced_example = Email::new("from@email.address", "to@email.address", "Body text")
//         .with_subject("Subject line")
//         .with_attachment(File::new(b"File!", "file-name.txt", &FileType::Txt));

//     match email_service.send(&email_advanced_example) {
//         Ok(()) => println!("Success!"),
//         Err(_) => println!("Error :("),
//     }
// }

// #[test]
// fn test_string() {
//     let email_service = OhMySmtp::new(format!("API_KEY"));

//     let _result = email_service.send(&Email::new(
//         format!("from@email.address"),
//         format!("to@email.address"),
//         format!("Body text"),
//     ));

//     let email_advanced_example = Email::new(
//         format!("from@email.address"),
//         format!("to@email.address"),
//         format!("Body text"),
//     )
//     .with_subject(format!("Subject line"))
//     .with_attachment(File::new(
//         b"File!",
//         format!("file-name.txt"),
//         &FileType::Txt,
//     ));

//     match email_service.send(&email_advanced_example) {
//         Ok(()) => println!("Success!"),
//         Err(_) => println!("Error :("),
//     }
// }

#[test]
#[cfg(feature = "email-validation")]
fn test_invalid_email() {
    use ohmysmtp::{Error, OhMySmtp};

    let email_service = OhMySmtp::new(format!("API_KEY"));

    assert_eq!(
        email_service.send(&Email::new(
            format!("from@email.address"),
            format!("test@-iana.org"),
            format!("Body text"),
        )),
        Err(Error::InvalidEmail)
    );
}
#[test]
fn test_basic_email() {
    let email = Email::new("test-from@email.org", "test-to@email.org", "Body text");
    assert_eq!(email.to_string(), "{\"from\":\"test-from@email.org\",\"to\":\"test-to@email.org\",\"textbody\":\"Body text\"}");
}
#[test]
fn test_basic_email_bcc() {
    let email = Email::new("test-from@email.org", "test-to@email.org", "Body text")
        .with_bcc("bcc@email.org");
    assert_eq!(email.to_string(), "{\"from\":\"test-from@email.org\",\"to\":\"test-to@email.org\",\"textbody\":\"Body text\",\"bcc\":\"bcc@email.org\"}");
}
#[test]
fn test_basic_email_html() {
    let email = Email::new("test-from@email.org", "test-to@email.org", "Body text")
        .with_html("<html><h1>Header</h1></html>");
    assert_eq!(email.to_string(), "{\"from\":\"test-from@email.org\",\"to\":\"test-to@email.org\",\"htmlbody\":\"<html><h1>Header</h1></html>\"}");
}

#[test]
fn test_basic_email_attachment() {
    let email = Email::new("test-from@email.org", "test-to@email.org", "Body text")
        .with_attachment(File::new(b"File!", "file-name.txt", &FileType::Txt));
    assert_eq!(email.to_string(), "{\"from\":\"test-from@email.org\",\"to\":\"test-to@email.org\",\"textbody\":\"Body text\",\"attachments\":[{\"name\":\"file-name.txt\",\"content\":\"RmlsZSE=\",\"content_type\":\"text/plain\"}]}");
}

#[test]
fn test_basic_email_listunsubscribe() {
    let email = Email::new("test-from@email.org", "test-to@email.org", "Body text")
        .with_list_unsubscribe("X-List-Unsubscribe: <http://www.host.com/list.cgi?cmd=unsub&lst=list>, <mailto:list-request@host.com?subject=unsubscribe>");
    assert_eq!(email.to_string(), "{\"from\":\"test-from@email.org\",\"to\":\"test-to@email.org\",\"textbody\":\"Body text\",\"list_unsubscribe\":\"X-List-Unsubscribe: <http://www.host.com/list.cgi?cmd=unsub&lst=list>, <mailto:list-request@host.com?subject=unsubscribe>\"}");
}
#[test]
fn test_basic_email_tag() {
    let email =
        Email::new("test-from@email.org", "test-to@email.org", "Body text").with_tag("tag-test");
    assert_eq!(email.to_string(), "{\"from\":\"test-from@email.org\",\"to\":\"test-to@email.org\",\"textbody\":\"Body text\",\"tags\":[\"tag-test\"]}");
}

#[test]
fn test_basic_email_tags() {
    let email = Email::new("test-from@email.org", "test-to@email.org", "Body text")
        .with_tags(vec!["tag-test", "tag-test-2"]);
    assert_eq!(email.to_string(), "{\"from\":\"test-from@email.org\",\"to\":\"test-to@email.org\",\"textbody\":\"Body text\",\"tags\":[\"tag-test\",\"tag-test-2\"]}");
}
