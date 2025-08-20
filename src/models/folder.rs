use crate::models::request::Request;

pub struct Folder {
    pub(crate) name: String,
    pub(crate) requests: Vec<Request>
}
