//! Ultra-easy API to the `OhMySMTP` service.
//! Yes, seriously - ultra-easy.
//! ```
//! use ohmysmtp::{Email, File, FileType, OhMySmtp};
//!
//! let email_service = OhMySmtp::new("API_KEY");
//!
//! let result = email_service.send(&Email::new(
//!     "from@email.address",
//!     "to@email.address",
//!     "Body text",
//! ));
//!
//!
//! let email_advanced_example =
//!     Email::new("from@email.address", "to@email.address", "Body text")
//!         .with_subject("Subject line")
//!         .with_attachment(File::new(b"File!", "file-name.txt", &FileType::Txt));
//! match email_service.send(&email_advanced_example) {
//!     Ok(()) => println!("Success!"),
//!     Err(e) => println!("Error :(")
//! }
//! ```

// #![warn(missing_docs)]
#![deny(
    anonymous_parameters,
    clippy::all,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_numeric_casts,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::nursery,
    clippy::pedantic,
    clippy::todo,
    clippy::unimplemented,
    clippy::use_debug,
    clippy::all,
    unused_qualifications,
    variant_size_differences
)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::needless_pass_by_value)] // Annoyingly, clippy kept recommending to change `impl ToString` to `&impl ToString`; doing that caused size-at-compile-time errors. :(

use std::fmt::Debug;

use nanoserde::SerJson;
use ureq::Response;

pub struct OhMySmtp {
    api_key: String,
    agent: ureq::Agent,
}

impl OhMySmtp {
    #[must_use]
    /// Create `OhMySmtp` instance with the given API key.
    pub fn new(api_key: impl ToString) -> Self {
        Self {
            api_key: api_key.to_string(),
            agent: ureq::AgentBuilder::new()
                .user_agent("gh:sigaloid/ohmysmtp-0.1.1")
                .build(),
        }
    }
    /// Send the given email with the API key of the `OhMySmtp` instance.
    /// # Errors
    /// Errors if any of the errors in the Errors enum are encountered.
    pub fn send(&self, email: &Email) -> Result<(), Error> {
        // If the email-validation feature is enabled, the email will be checked before using.
        #[cfg(feature = "email-validation")]
        {
            if email_address_parser::EmailAddress::parse(&email.to, None).is_none() {
                return Err(Error::InvalidEmail);
            }
        }
        // Create an empty POST request to the endpoint
        let request = self.agent.post("https://app.ohmysmtp.com/api/v1/send");
        // Serialize the `email` object to string
        let email_json_string = nanoserde::SerJson::serialize_json(email);
        // Helper function to parse the response from the server, given the status code and the response body.
        // Bodged from `https://docs.ohmysmtp.com/reference/responses`
        let read_status = |status_code: u16, response: Response| match status_code {
            200 => Ok(()),
            400 => {
                if let Ok(response_string) = response.into_string() {
                    if response_string.contains("Invalid API") {
                        return Err(Error::InvalidApiToken);
                    } else if response_string.contains("not parseable") {
                        return Err(Error::FromAddressNotParseable);
                    } else if response_string.contains("undefined field") {
                        return Err(Error::NoToField);
                    } else if response_string.contains("is invalid") {
                        return Err(Error::ToAddressNotParseable);
                    } else if response_string.contains("blocked address") {
                        return Err(Error::ToAddressBlocked);
                    } else if response_string.contains("maximum volume") {
                        return Err(Error::RateLimit);
                    } else if response_string.contains("Extension file type blocked") {
                        return Err(Error::ExtensionTypeBlocked);
                    }
                    return Err(Error::Other(response_string));
                }
                Err(Error::Other(status_code.to_string()))
            }
            401 => Err(Error::MissingApiToken),
            403 => {
                if let Ok(response_string) = response.into_string() {
                    if response_string.contains("Domain DKIM") {
                        return Err(Error::DomainDkimVerificationNotCompleted);
                    }
                    if response_string.contains("not have an active plan") {
                        return Err(Error::InactivePlanForDomain);
                    }
                    if response_string.contains("unable to send email") {
                        return Err(Error::OrganizationDisabled);
                    }
                    if response_string.contains("Verified domain") {
                        return Err(Error::FromAddressNotEqualToRegisteredDomain);
                    }
                    return Err(Error::Other(response_string));
                }
                Err(Error::Other(status_code.to_string()))
            }
            406 => Err(Error::InvalidRequestFormat),
            429 => Err(Error::RateLimit),
            500 => Err(Error::NoContent),
            _ => Err(Error::Other(
                response
                    .into_string()
                    .unwrap_or_else(|_| status_code.to_string()),
            )),
        };
        // Send response
        let response = request
            .set("Accept", "application/json")
            .set("Content-Type", "application/json")
            .set("OhMySMTP-Server-Token", &self.api_key)
            .send_string(&email_json_string);
        // Match on the response, short circuit if `ureq::Error` is returned.
        match response {
            Ok(response) => {
                let status_code = response.status();
                read_status(status_code, response)
            }
            Err(ureq::Error::Status(code, response)) => {
                let status_code = code;
                read_status(status_code, response)
            }
            Err(error) => Err(Error::NetworkError(error.to_string())),
        }
    }
}
#[derive(Debug, SerJson, Clone, Default)]
pub struct Email {
    from: String,
    to: String,
    #[nserde(rename = "textbody")]
    text_body: Option<String>,
    #[nserde(rename = "htmlbody")]
    html_body: Option<String>,
    cc: Option<String>,
    bcc: Option<String>,
    subject: Option<String>,
    #[nserde(rename = "replyto")]
    reply_to: Option<String>,
    list_unsubscribe: Option<String>,
    attachments: Option<Vec<File>>,
    tags: Option<Vec<String>>,
}
impl ToString for Email {
    // Implemented for tests
    fn to_string(&self) -> String {
        nanoserde::SerJson::serialize_json(self)
    }
}
impl Email {
    #[must_use]
    /// Create a new Email object
    pub fn new(from: impl ToString, to: impl ToString, body: impl ToString) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            text_body: Some(body.to_string()),
            ..Self::default()
        }
    }
    #[must_use]
    /// Include an HTML body to the email.
    pub fn with_html(mut self, html_body: impl ToString) -> Self {
        self.html_body = Some(html_body.to_string());
        self.text_body = None;
        self
    }
    #[must_use]
    /// Include a text body to the email.
    pub fn with_text_body(mut self, textbody: impl ToString) -> Self {
        self.text_body = Some(textbody.to_string());
        self.html_body = None;
        self
    }
    #[must_use]
    /// Send a cc (carbon copy) with the email, to the provided address.
    pub fn with_cc(mut self, cc: impl ToString) -> Self {
        self.cc = Some(cc.to_string());
        self
    }
    #[must_use]
    /// Send a bcc (blind carbon copy) with the email, to the provided address.
    pub fn with_bcc(mut self, bcc: impl ToString) -> Self {
        self.bcc = Some(bcc.to_string());
        self
    }
    #[must_use]
    /// Include subject with email.
    pub fn with_subject(mut self, subject: impl ToString) -> Self {
        self.subject = Some(subject.to_string());
        self
    }
    #[must_use]
    /// Include reply-to header containing the given email address with the email
    pub fn with_replyto(mut self, replyto: impl ToString) -> Self {
        self.reply_to = Some(replyto.to_string());
        self
    }
    #[must_use]
    /// Include a list-unsubscribe header with the email
    pub fn with_list_unsubscribe(mut self, listunsubscribe: impl ToString) -> Self {
        self.list_unsubscribe = Some(listunsubscribe.to_string());
        self
    }
    #[must_use]
    /// Include a list of attachments to the email (note: this will clear any previusly added attachments!)
    pub fn with_attachments(mut self, attachments: Vec<File>) -> Self {
        self.attachments = Some(attachments);
        self
    }
    #[must_use]
    /// Include an attachment to the email (note: this will clear any previously added attachments!)
    pub fn with_attachment(mut self, attachment: File) -> Self {
        self.attachments = Some(vec![attachment]);
        self
    }

    #[must_use]
    /// Include a list of tags for the email
    pub fn with_tags(mut self, tags: Vec<impl ToString>) -> Self {
        self.tags = Some(tags.into_iter().map(|x| x.to_string()).collect());
        self
    }
    #[must_use]
    /// Include a tag for the email (for internal use within the `OhMySMTP` service)
    pub fn with_tag(mut self, tag: impl ToString) -> Self {
        self.tags = Some(vec![tag.to_string()]);
        self
    }
}
#[derive(Debug, SerJson, Clone)]
pub struct File {
    name: String,
    content: String,
    content_type: String,
    cid: Option<String>,
}
impl File {
    /// Create new `File` by reading the bytes and setting the filetype and filename.
    pub fn new(bytes: &dyn AsRef<[u8]>, name: impl ToString, filetype: &FileType) -> Self {
        Self {
            name: name.to_string(),
            // Content is base64-encoded as per OhMySMTP docs
            content: base64::encode(bytes),
            content_type: match filetype {
                FileType::Jpeg | FileType::Jpg => "image/jpeg".into(),
                FileType::Png => "image/png".into(),
                FileType::Gif => "image/gif".into(),
                FileType::Txt => "text/plain".into(),
                FileType::Pdf => "application/pdf".into(),
                FileType::Docx => {
                    "application/vnd.openxmlformats-officedocument.wordprocessingml.document".into()
                }
                FileType::Xlsx => {
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".into()
                }
                FileType::Pptx => {
                    "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                        .into()
                }
                FileType::Csv => "text/csv".into(),
            },
            cid: None,
        }
    }
}

/// These filetypes are the only ones allowed as per `https://docs.ohmysmtp.com/reference/send/`
pub enum FileType {
    Jpeg,
    Jpg,
    Png,
    Gif,
    Txt,
    Pdf,
    Docx,
    Xlsx,
    Pptx,
    Csv,
    // att
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// We can't match your API token to a Domain
    InvalidApiToken,
    /// Our API is unable to parse the email address you are sending from
    FromAddressNotParseable,
    /// You did not include a To field in your request
    NoToField,
    /// The To field does not contain a valid email address
    ToAddressNotParseable,
    /// An email in the To field is in your blocked addresses list, which we cannot send to
    ToAddressBlocked,
    /// You can send up to 50 emails in one go by including them in the To field, this request has more than 50 emails in the To field
    TooManyToAddrs,
    /// See [here](https://docs.ohmysmtp.com/reference/send/) for details of allowed attachment file types
    ExtensionTypeBlocked,
    /// You must include an API token in every request
    MissingApiToken,
    /// Every domain must complete DKIM verification before emails can be sent from it
    DomainDkimVerificationNotCompleted,
    /// Each organization must have an active plan to allow emails to be sent
    InactivePlanForDomain,
    /// Your organization has been disabled. Please contact support via email for details: support@ohmysmtp.com
    OrganizationDisabled,
    /// The From address needs to contain exactly the same domain that you have registered, for example, if the email has a From address of test@test.com, you must be attempting to send using the API token for the test.com address
    FromAddressNotEqualToRegisteredDomain,
    /// Something in your request is invalid, check the (Send Reference Documentation)[send] for details
    InvalidRequestFormat,
    /// You are being rate limited due to sending too many emails in a short period of time. The application of rate limits varies depending on factors such as organization age, plan, and historical sending patterns. Contact Support if you are experiencing this regularly
    RateLimit,
    /// Internal Server Error - our application is down, contact support if this persists
    NoContent,
    /// Network error - the server could not be reached
    NetworkError(String),
    /// Other
    Other(String),
    #[cfg(feature = "email-validation")]
    /// Error within email validation (previous to any network requests to OhMySmtp)
    InvalidEmail,
}
