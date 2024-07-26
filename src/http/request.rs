use super::method::Method;
pub struct Request {
    path: String,
    query_string: Option<String>, //Option enum because some times query params doesnot exist
    method: Method, //super means go one level up(http) then find the method module
}