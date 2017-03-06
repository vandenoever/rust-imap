use std::io::Error as IoError;
use std::result;
use std::fmt;
use std::error::Error as StdError;
use std::net::TcpStream;
use std::string::FromUtf8Error;

use openssl::ssl::HandshakeError as SslError;

pub type Result<T> = result::Result<T, Error>;

/// A set of errors that can occur in the IMAP client
#[derive(Debug)]
pub enum Error {
    /// An `io::Error` that occurred while trying to read or write to a network stream.
    Io(IoError),
    /// An error from the `openssl` library.
    Ssl(SslError<TcpStream>),
    /// A BAD response from the IMAP server.
    BadResponse(Vec<String>),
    /// A NO response from the IMAP server.
    NoResponse(Vec<String>),
    // Error parsing a server response.
    Parse(ParseError)
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<SslError<TcpStream>> for Error {
    fn from(err: SslError<TcpStream>) -> Error {
        Error::Ssl(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Parse(ParseError::FromUtf8(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => fmt::Display::fmt(e, f),
            Error::Ssl(ref e) => fmt::Display::fmt(e, f),
            ref e => f.write_str(e.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Ssl(ref e) => e.description(),
            Error::Parse(ref e) => e.description(),
            Error::BadResponse(_) => "Bad Response",
            Error::NoResponse(_) => "No Response",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Ssl(ref e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
	// Error in the decoding of data.
	FromUtf8(FromUtf8Error),
    // Indicates an error parsing the status response. Such as OK, NO, and BAD.
    StatusResponse(Vec<String>),
    // Error parsing the cabability response.
    Capability(Vec<String>),
    // Authentication errors.
    Authentication(String)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ref e => f.write_str(e.description()),
        }
    }
}

impl StdError for ParseError {
    fn description(&self) -> &str {
        match *self {
        	ParseError::FromUtf8(_) => "Unable to decode the response as UTF-8.",
            ParseError::StatusResponse(_) => "Unable to parse status response",
            ParseError::Capability(_) => "Unable to parse capability response",
            ParseError::Authentication(_) => "Unable to parse authentication response"
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            _ => None
        }
    }
}
