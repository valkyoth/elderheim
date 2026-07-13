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
    pub text_vaddr: u64,
    pub text_size: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutError {
    FileRangeOverflow,
    VirtualAddressRangeOverflow,
    TextOutsideFile,
    EntryOutsideText,
}

impl ImagePlan {
    pub const fn validate(self) -> Result<(), LayoutError> {
        let Some(text_file_end) = self.text_offset.checked_add(self.text_size) else {
            return Err(LayoutError::FileRangeOverflow);
        };
        let Some(text_vaddr_end) = self.text_vaddr.checked_add(self.text_size) else {
            return Err(LayoutError::VirtualAddressRangeOverflow);
        };

        if text_file_end > self.file_size {
            return Err(LayoutError::TextOutsideFile);
        }
        if self.entry_vaddr < self.text_vaddr || self.entry_vaddr >= text_vaddr_end {
            return Err(LayoutError::EntryOutsideText);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ELF_MAGIC, ImagePlan, LayoutError};

    #[test]
    fn magic_is_elf() {
        assert_eq!(ELF_MAGIC, [0x7f, b'E', b'L', b'F']);
    }

    #[test]
    fn layout_validation_keeps_file_and_virtual_ranges_separate() {
        let plan = ImagePlan {
            file_size: 0x2000,
            entry_vaddr: 0x401000,
            text_offset: 0x1000,
            text_vaddr: 0x401000,
            text_size: 0x100,
        };
        assert_eq!(plan.validate(), Ok(()));
    }

    #[test]
    fn layout_validation_rejects_file_and_address_overflow() {
        let file_overflow = ImagePlan {
            file_size: u64::MAX,
            entry_vaddr: 0x1000,
            text_offset: u64::MAX,
            text_vaddr: 0x1000,
            text_size: 1,
        };
        assert_eq!(
            file_overflow.validate(),
            Err(LayoutError::FileRangeOverflow)
        );

        let address_overflow = ImagePlan {
            file_size: 2,
            entry_vaddr: u64::MAX,
            text_offset: 0,
            text_vaddr: u64::MAX,
            text_size: 1,
        };
        assert_eq!(
            address_overflow.validate(),
            Err(LayoutError::VirtualAddressRangeOverflow)
        );
    }

    #[test]
    fn layout_validation_rejects_out_of_bounds_text_and_entry() {
        let text_outside_file = ImagePlan {
            file_size: 0x1080,
            entry_vaddr: 0x401000,
            text_offset: 0x1000,
            text_vaddr: 0x401000,
            text_size: 0x100,
        };
        assert_eq!(
            text_outside_file.validate(),
            Err(LayoutError::TextOutsideFile)
        );

        for entry_vaddr in [0x400fff, 0x401100] {
            let entry_outside_text = ImagePlan {
                file_size: 0x2000,
                entry_vaddr,
                text_offset: 0x1000,
                text_vaddr: 0x401000,
                text_size: 0x100,
            };
            assert_eq!(
                entry_outside_text.validate(),
                Err(LayoutError::EntryOutsideText)
            );
        }
    }
}
