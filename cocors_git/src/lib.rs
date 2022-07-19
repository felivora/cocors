pub mod author;

pub use crate::author::Author;
pub mod command;
pub mod commit;

#[cfg(test)]
mod tests {
    use crate::command;
    #[test]
    fn is_git_installed() {
        assert_eq!(command::installed(), true);
    }
}
