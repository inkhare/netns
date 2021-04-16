pub mod errno;
pub mod netns;
pub mod netns_linux;

pub use errno::{Errno, Error};
pub use netns::{apply_in_ns, ExecStatus};
pub use netns_linux::NetNS;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
