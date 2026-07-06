#![no_std]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LabelId(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataId(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MirOp {
    Label(LabelId),
    WriteStatic(DataId),
    Exit { code: u8 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LirOp {
    Label(LabelId),
    SysWriteStatic(DataId),
    SysExit { code: u8 },
}

#[cfg(test)]
mod tests {
    use super::{DataId, MirOp};

    #[test]
    fn write_static_is_target_neutral() {
        assert_eq!(MirOp::WriteStatic(DataId(7)), MirOp::WriteStatic(DataId(7)));
    }
}
