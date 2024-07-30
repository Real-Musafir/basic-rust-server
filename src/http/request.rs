use super::method::Method;
use std::convert::TryFrom;
pub struct Request {
    path: String,
    query_string: Option<String>, //Option enum because some times query params doesnot exist
    method: Method,               //super means go one level up(http) then find the method module
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    // Get /search?name=abc&sort=1 HTTP/1.1

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
