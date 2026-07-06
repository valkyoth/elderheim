#![no_std]

pub use elderheim_backend_x86 as backend_x86;
pub use elderheim_core as core;
pub use elderheim_dartmouth_basic as dartmouth_basic;
pub use elderheim_format_elf as format_elf;
pub use elderheim_ir as ir;
pub use elderheim_target as target;

pub const PROJECT_NAME: &str = "Elderheim";

#[cfg(test)]
mod tests {
    use super::PROJECT_NAME;

    #[test]
    fn exposes_project_name() {
        assert_eq!(PROJECT_NAME, "Elderheim");
    }
}
