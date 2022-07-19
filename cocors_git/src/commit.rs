use crate::Author;
use chrono::{DateTime, Utc};

#[derive(Eq, PartialEq, Debug)]
pub struct Commit {
    pub hash: String,
    pub author: Author,
    pub timestamp: DateTime<Utc>,
    pub subject: String,
    pub body: Option<String>,
}
