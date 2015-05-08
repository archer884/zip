pub enum ZipError {
    Input,
    API,
    Invalid(String),
}

impl ::std::fmt::Display for ZipError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            &ZipError::Input => f.write_str("No zip provided"),
            &ZipError::API => f.write_str("API unavailable"),
            &ZipError::Invalid(ref candidate) => write!(f, "Invalid zip: {}", candidate),
        }
    }
}
