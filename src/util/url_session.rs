//! API requests through the system's shared `URLSession`. It keeps connections alive between
//! dictations and speaks HTTP/3 where the server offers it, which saves the connection setup —
//! over a second per request on slow links. It also follows the system proxy settings.

use std::fmt;
use std::sync::mpsc;
use std::time::Duration;

use block2::RcBlock;
use objc2_foundation::{
    NSData, NSError, NSHTTPURLResponse, NSMutableURLRequest, NSString, NSURL, NSURLErrorDomain,
    NSURLErrorTimedOut, NSURLResponse, NSURLSession, ns_string,
};

pub struct Response {
    /// `None` when the answer was not an HTTP response.
    pub status: Option<u16>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
}

#[derive(Debug)]
pub enum RequestError {
    InvalidUrl,
    TimedOut,
    /// URLSession's localised description, e.g. for a missing connection or a TLS failure.
    Failed(String),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUrl => f.write_str("invalid URL"),
            Self::TimedOut => f.write_str("request timed out"),
            Self::Failed(description) => f.write_str(description),
        }
    }
}

impl std::error::Error for RequestError {}

/// Sends API requests; tests replace it with recorded answers.
pub trait HttpClient {
    /// Sends a POST and waits for the answer; `timeout` bounds the whole request.
    fn post(
        &self,
        url: &str,
        headers: &[(&str, &str)],
        body: &[u8],
        timeout: Duration,
    ) -> Result<Response, RequestError>;
}

pub struct UrlSession;

impl HttpClient for UrlSession {
    fn post(
        &self,
        url: &str,
        headers: &[(&str, &str)],
        body: &[u8],
        timeout: Duration,
    ) -> Result<Response, RequestError> {
        post(url, headers, body, timeout)
    }
}

/// Opens a connection to the URL's host for the next request and ignores the answer.
pub fn warm_up(url: &str) {
    let Some(url) = NSURL::URLWithString(&NSString::from_str(url)) else {
        return;
    };
    let request = NSMutableURLRequest::requestWithURL(&url);
    request.setHTTPMethod(ns_string!("HEAD"));
    request.setTimeoutInterval(10.0);
    let handler = RcBlock::new(|_: *mut NSData, _: *mut NSURLResponse, _: *mut NSError| {});
    // SAFETY: the handler does nothing with its arguments.
    let task = unsafe {
        NSURLSession::sharedSession().dataTaskWithRequest_completionHandler(&request, &handler)
    };
    task.resume();
}

/// URLSession ends a request on its own when no data arrives within the request's timeout interval.
fn request_error(error: &NSError) -> RequestError {
    // SAFETY: a constant that Foundation initialises before any code runs.
    let url_domain = unsafe { NSURLErrorDomain };
    if error.code() == NSURLErrorTimedOut && error.domain().isEqualToString(url_domain) {
        RequestError::TimedOut
    } else {
        RequestError::Failed(error.localizedDescription().to_string())
    }
}

fn post(
    url: &str,
    headers: &[(&str, &str)],
    body: &[u8],
    timeout: Duration,
) -> Result<Response, RequestError> {
    let url = NSURL::URLWithString(&NSString::from_str(url)).ok_or(RequestError::InvalidUrl)?;
    let request = NSMutableURLRequest::requestWithURL(&url);
    request.setHTTPMethod(ns_string!("POST"));
    for (name, value) in headers {
        request.setValue_forHTTPHeaderField(
            Some(&NSString::from_str(value)),
            &NSString::from_str(name),
        );
    }
    request.setHTTPBody(Some(&NSData::with_bytes(body)));
    request.setTimeoutInterval(timeout.as_secs_f64());

    let (sender, receiver) = mpsc::channel();
    let handler = RcBlock::new(
        move |data: *mut NSData, response: *mut NSURLResponse, error: *mut NSError| {
            // SAFETY: URLSession passes valid objects or null for the duration of the handler.
            let (data, response, error) =
                unsafe { (data.as_ref(), response.as_ref(), error.as_ref()) };
            let result = match error {
                Some(error) => Err(request_error(error)),
                None => Ok(Response {
                    status: response
                        .and_then(|response| response.downcast_ref::<NSHTTPURLResponse>())
                        .and_then(|response| u16::try_from(response.statusCode()).ok()),
                    body: data.map(NSData::to_vec).unwrap_or_default(),
                }),
            };
            let _ = sender.send(result);
        },
    );
    // SAFETY: the handler only moves plain Rust data out through a channel.
    let task = unsafe {
        NSURLSession::sharedSession().dataTaskWithRequest_completionHandler(&request, &handler)
    };
    task.resume();
    if let Ok(result) = receiver.recv_timeout(timeout) {
        result
    } else {
        task.cancel();
        Err(RequestError::TimedOut)
    }
}

#[cfg(test)]
mod tests {
    use objc2_foundation::NSCocoaErrorDomain;

    use super::*;

    #[test]
    fn urlsession_timeouts_are_timeouts() {
        // SAFETY: constants that Foundation initialises before any code runs.
        let (url_domain, cocoa_domain) = unsafe { (NSURLErrorDomain, NSCocoaErrorDomain) };
        assert!(matches!(
            request_error(&NSError::new(NSURLErrorTimedOut, url_domain)),
            RequestError::TimedOut
        ));
        assert!(matches!(
            request_error(&NSError::new(-1009, url_domain)),
            RequestError::Failed(_)
        ));
        assert!(matches!(
            request_error(&NSError::new(NSURLErrorTimedOut, cocoa_domain)),
            RequestError::Failed(_)
        ));
    }
}
