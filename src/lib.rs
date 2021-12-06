// #![warn(missing_docs)]
#![deny(
    anonymous_parameters,
    clippy::all,
    const_err,
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

use nanoserde::{DeJson, SerJson};
use std::fmt::Debug;
use ureq::Response;

pub struct OhMySmtp {
    api_key: String,
}
impl OhMySmtp {
    #[must_use]
    /// Create OhMySmtp instance with the given API key.
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }
    /// Send the given email with the API key of the OhMySmtp instance.
    pub fn send_email(&self, email: &Email) -> Result<(), Error> {
        let request = ureq::post("https://app.ohmysmtp.com/api/v1/send");
        let str = nanoserde::SerJson::serialize_json(email);
        // println!("{}", &str); // Debugging
        let read_status = |status: u16, response: Response| match status {
            200 => Ok(()),
            400 => {
                if let Ok(response_string) = response.into_string() {
                    if response_string.contains("Invalid API") {
                        return Err(Error::InvalidApiToken);
                    } else if response_string.contains("not parseable") {
                        return Err(Error::FromAddrNotParseable);
                    } else if response_string.contains("undefined field") {
                        return Err(Error::NoToField);
                    } else if response_string.contains("is invalid") {
                        return Err(Error::ToAddrNotParseable);
                    } else if response_string.contains("blocked address") {
                        return Err(Error::ToAddrBlocked);
                    } else if response_string.contains("maximum volume") {
                        return Err(Error::RateLimit);
                    } else if response_string.contains("Extension file type blocked") {
                        return Err(Error::ExtensionTypeBlocked);
                    }
                    return Err(Error::Other(response_string));
                }
                Err(Error::Other(status.to_string()))
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
                        return Err(Error::FromAddrNotEqualToRegisteredDomain);
                    }
                    return Err(Error::Other(response_string));
                }
                Err(Error::Other(status.to_string()))
            }
            406 => Err(Error::InvalidRequestFormat),
            429 => Err(Error::RateLimit),
            500 => Err(Error::NoContent),
            _ => Err(Error::Other(
                response
                    .into_string()
                    .unwrap_or_else(|_| status.to_string()),
            )),
        };
        match request
            .set("Accept", "application/json")
            .set("Content-Type", "application/json")
            .set("OhMySMTP-Server-Token", &self.api_key)
            .send_string(&str)
        {
            Ok(response) => {
                let status = response.status();
                read_status(status, response)
            }
            Err(ureq::Error::Status(code, response)) => {
                let status = code;
                read_status(status, response)
            }
            Err(error) => Err(Error::NetworkError(Box::new(error))),
        }
    }
}
#[derive(Debug, DeJson, SerJson)]
pub struct Email {
    from: String,
    to: String,
    textbody: Option<String>,
    htmlbody: Option<String>,
    cc: Option<String>,
    bcc: Option<String>,
    subject: Option<String>,
    replyto: Option<String>,
    list_unsubscribe: Option<String>,
    attachments: Option<Vec<File>>,
    tags: Option<Vec<String>>,
}
impl Default for Email {
    fn default() -> Self {
        Self {
            from: "".into(),
            to: "".into(),
            textbody: None,
            cc: None,
            bcc: None,
            subject: None,
            replyto: None,
            list_unsubscribe: None,
            attachments: None,
            tags: None,
            htmlbody: None,
        }
    }
}
impl Email {
    #[must_use]
    /// Create a new Email object
    pub fn new(from: String, to: String, body: String) -> Self {
        Self {
            from,
            to,
            textbody: Some(body),
            ..Self::default()
        }
    }
    #[must_use]
    /// Include an HTML body to the email.
    pub fn with_html(mut self, html_body: String) -> Self {
        self.htmlbody = Some(html_body);
        self.textbody = None;
        self
    }
    #[must_use]
    /// Include a text body to the email.
    pub fn with_text_body(mut self, textbody: String) -> Self {
        self.textbody = Some(textbody);
        self.htmlbody = None;
        self
    }
    #[must_use]
    /// Send a cc (carbon copy) with the email, to the provided address.
    pub fn with_cc(mut self, cc: String) -> Self {
        self.cc = Some(cc);
        self
    }
    #[must_use]
    /// Send a bcc (blind carbon copy) with the email, to the provided address.
    pub fn with_bcc(mut self, bcc: String) -> Self {
        self.bcc = Some(bcc);
        self
    }
    #[must_use]
    /// Include subject with email.
    pub fn with_subject(mut self, subject: String) -> Self {
        self.subject = Some(subject);
        self
    }
    #[must_use]
    /// Include reply-to header containing the given email address with the email
    pub fn with_replyto(mut self, replyto: String) -> Self {
        self.replyto = Some(replyto);
        self
    }
    #[must_use]
    /// Include a list-unsubscribe header with the email
    pub fn with_list_unsubscribe(mut self, listunsubscribe: String) -> Self {
        self.list_unsubscribe = Some(listunsubscribe);
        self
    }
    #[must_use]
    /// Include a list of attachments to the email
    pub fn with_attachments(mut self, attachments: Vec<File>) -> Self {
        self.attachments = Some(attachments);
        self
    }
    #[must_use]
    /// Include a list of tags for the email
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
}
#[derive(Debug, DeJson, SerJson)]
pub struct File {
    name: String,
    bytes: Vec<u8>,
    content_type: String,
    cid: Option<String>,
}
impl File {
    // Convert &OsStr to a Result<File, Error> by reading the bytes and guessing the MIME type.
    // (WIP)
    // pub fn from_file_path(file_path: &OsStr) -> Result<Self, Error> {
    //     let name = file_path.to_str()?.to_string();
    //     let bytes = std::fs::File::open(file_path).bytes();
    //     let content_type = new_mime_guess::from_path(file_path)
    //         .first()
    //         .ok()?
    //         .to_string();
    //     Ok(Self {
    //         name,
    //         bytes,
    //         content_type: content_type,
    //         cid: None,
    //     })
    // }
}
#[derive(Debug)]
pub enum Error {
    /// We can't match your API token to a Domain
    InvalidApiToken,
    /// Our API is unable to parse the email address you are sending from
    FromAddrNotParseable,
    /// You did not include a To field in your request
    NoToField,
    /// The To field does not contain a valid email address
    ToAddrNotParseable,
    /// An email in the To field is in your blocked addresses list, which we cannot send to
    ToAddrBlocked,
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
    FromAddrNotEqualToRegisteredDomain,
    /// Something in your request is invalid, check the (Send Reference Documentation)[send] for details
    InvalidRequestFormat,
    /// You are being rate limited due to sending too many emails in a short period of time. The application of rate limits varies depending on factors such as organization age, plan, and historical sending patterns. Contact Support if you are experiencing this regularly
    RateLimit,
    /// Internal Server Error - our application is down, contact support if this persists
    NoContent,
    /// Network error - the server could not be reached
    NetworkError(Box<ureq::Error>),
    ///Other
    Other(String),
}