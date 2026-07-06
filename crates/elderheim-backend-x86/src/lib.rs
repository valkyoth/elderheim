#![no_std]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Reg64 {
    Rax,
    Rdi,
    Rsi,
    Rdx,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelocKind {
    RipRel32,
    BranchRel32,
    CallRel32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Reloc {
    pub patch_at: u32,
    pub instruction_end: u32,
    pub kind: RelocKind,
}

#[must_use]
pub const fn syscall_bytes() -> [u8; 2] {
    [0x0f, 0x05]
}

#[cfg(test)]
mod tests {
    use super::syscall_bytes;

    #[test]
    fn syscall_encoding_is_stable() {
        assert_eq!(syscall_bytes(), [0x0f, 0x05]);
    }
}
