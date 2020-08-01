/// Errors that can occur with the networking library
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Unknown error. Please report this number to the maintainers of this crate when it occurs, so a proper error variant can be implemented
    Unknown(i32),
}
