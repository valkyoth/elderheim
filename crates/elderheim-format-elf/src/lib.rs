#![no_std]

pub const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
pub const ELF_CLASS_64: u8 = 2;
pub const ELF_DATA_LSB: u8 = 1;
pub const EM_X86_64: u16 = 0x3e;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SegmentPerms {
    Read,
    ReadExecute,
    ReadWrite,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImagePlan {
    pub file_size: u64,
    pub entry_vaddr: u64,
    pub text_offset: u64,
    pub text_size: u64,
}

impl ImagePlan {
    #[must_use]
    pub const fn is_entry_in_text(self) -> bool {
        self.entry_vaddr >= self.text_offset
            && self.entry_vaddr < self.text_offset.saturating_add(self.text_size)
    }
}

#[cfg(test)]
mod tests {
    use super::{ELF_MAGIC, ImagePlan};

    #[test]
    fn magic_is_elf() {
        assert_eq!(ELF_MAGIC, [0x7f, b'E', b'L', b'F']);
    }

    #[test]
    fn entry_validation_checks_text_range() {
        let plan = ImagePlan {
            file_size: 256,
            entry_vaddr: 128,
            text_offset: 120,
            text_size: 16,
        };
        assert!(plan.is_entry_in_text());
    }
}
