use serde::Serialize;
use std::fmt::Debug;

pub struct Response<T: Serialize + Debug> {
    pub api_response: reqwest::Response,
    pub data: Box<T>,
}

impl<T: Serialize + Debug> Response<T> {
    pub fn new(api_response: reqwest::Response, data: T) -> Response<T> {
        Response {
            api_response,
            data: Box::new(data),
        } 
    }
}
