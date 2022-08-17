/// Type of http response
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ResponseType {
    /// Plain http
    Http,
    /// HAP event
    Event
}
