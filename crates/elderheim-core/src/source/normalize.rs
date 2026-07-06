use super::SourceError;
use crate::CompileLimits;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceId(pub u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlankLinePolicy {
    Preserve,
    Reject,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NormalizationPolicy {
    pub blank_lines: BlankLinePolicy,
}

impl NormalizationPolicy {
    pub const BASIC_STRICT: Self = Self {
        blank_lines: BlankLinePolicy::Reject,
    };

    pub const PRESERVE_BLANK_LINES: Self = Self {
        blank_lines: BlankLinePolicy::Preserve,
    };
}

pub trait NormalizedSourceSink {
    /// An error invalidates all bytes written to the sink during this call.
    fn push_byte(&mut self, byte: u8) -> Result<(), SourceError>;
}

pub fn normalize_source(
    bytes: &[u8],
    limits: CompileLimits,
    policy: NormalizationPolicy,
    sink: &mut dyn NormalizedSourceSink,
) -> Result<SourceId, SourceError> {
    validate_len(bytes, limits)?;

    let mut id = SourceId(FNV_OFFSET);
    let max_lines = u32::try_from(limits.max_lines).map_err(|_| SourceError::LimitTooLarge)?;
    let mut scanner = NormalizedSourceScanner::new(bytes.is_empty(), max_lines)?;
    let mut index = 0_usize;

    while let Some(byte) = bytes.get(index).copied() {
        let offset = u32::try_from(index).map_err(|_| SourceError::SourceTooLarge)?;

        match byte {
            b'\r' => {
                let next_index = index.saturating_add(1);
                if matches!(bytes.get(next_index), Some(b'\n')) {
                    index = next_index;
                }
                scanner.accept_normalized_byte(b'\n', offset, policy)?;
                push_normalized(b'\n', sink, &mut id)?;
            }
            b'\n' => {
                scanner.accept_normalized_byte(b'\n', offset, policy)?;
                push_normalized(b'\n', sink, &mut id)?;
            }
            _ => {
                scanner.accept_normalized_byte(byte, offset, policy)?;
                push_normalized(byte, sink, &mut id)?;
            }
        }

        index = index.saturating_add(1);
    }

    scanner.finish(policy)?;

    Ok(id)
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

fn validate_len(bytes: &[u8], limits: CompileLimits) -> Result<(), SourceError> {
    let max_supported_len = usize::try_from(u32::MAX).map_err(|_| SourceError::LimitTooLarge)?;
    if bytes.len() > max_supported_len {
        return Err(SourceError::SourceTooLarge);
    }

    if bytes.len() > limits.max_source_bytes {
        return Err(SourceError::SourceTooLarge);
    }

    Ok(())
}

pub(super) fn validate_normalized_source(
    bytes: &[u8],
    limits: CompileLimits,
    policy: NormalizationPolicy,
) -> Result<u32, SourceError> {
    validate_len(bytes, limits)?;

    let max_lines = u32::try_from(limits.max_lines).map_err(|_| SourceError::LimitTooLarge)?;
    let mut scanner = NormalizedSourceScanner::new(bytes.is_empty(), max_lines)?;
    for (index, byte) in bytes.iter().copied().enumerate() {
        let offset = u32::try_from(index).map_err(|_| SourceError::SourceTooLarge)?;
        scanner.accept_normalized_byte(byte, offset, policy)?;
    }
    scanner.finish(policy)
}

struct NormalizedSourceScanner {
    line: u32,
    line_count: u32,
    max_lines: u32,
    line_has_text: bool,
    ended_with_line_ending: bool,
}

impl NormalizedSourceScanner {
    fn new(is_empty: bool, max_lines: u32) -> Result<Self, SourceError> {
        let line_count = if is_empty { 0_u32 } else { 1_u32 };
        if line_count > max_lines {
            return Err(SourceError::TooManyLines);
        }

        Ok(Self {
            line: 1,
            line_count,
            max_lines,
            line_has_text: false,
            ended_with_line_ending: false,
        })
    }

    fn accept_normalized_byte(
        &mut self,
        byte: u8,
        offset: u32,
        policy: NormalizationPolicy,
    ) -> Result<(), SourceError> {
        match byte {
            b'\n' => self.accept_line_ending(policy),
            0x20..=0x7e => {
                if byte != b' ' {
                    self.line_has_text = true;
                }
                self.ended_with_line_ending = false;
                Ok(())
            }
            _ => Err(SourceError::InvalidByte { offset, byte }),
        }
    }

    fn accept_line_ending(&mut self, policy: NormalizationPolicy) -> Result<(), SourceError> {
        finish_line(policy, self.line, self.line_has_text)?;
        self.line_count = self
            .line_count
            .checked_add(1)
            .ok_or(SourceError::LocationOverflow)?;
        if self.line_count > self.max_lines {
            return Err(SourceError::TooManyLines);
        }
        self.line = self
            .line
            .checked_add(1)
            .ok_or(SourceError::LocationOverflow)?;
        self.line_has_text = false;
        self.ended_with_line_ending = true;
        Ok(())
    }

    fn finish(self, policy: NormalizationPolicy) -> Result<u32, SourceError> {
        if self.line_count != 0 && !self.ended_with_line_ending {
            finish_line(policy, self.line, self.line_has_text)?;
        }
        Ok(self.line_count)
    }
}

fn finish_line(
    policy: NormalizationPolicy,
    line: u32,
    line_has_text: bool,
) -> Result<(), SourceError> {
    if policy.blank_lines == BlankLinePolicy::Reject && !line_has_text {
        Err(SourceError::BlankLine { line })
    } else {
        Ok(())
    }
}

fn push_normalized(
    byte: u8,
    sink: &mut dyn NormalizedSourceSink,
    id: &mut SourceId,
) -> Result<(), SourceError> {
    sink.push_byte(byte)?;
    update_source_id(id, byte);
    Ok(())
}

pub(super) fn source_id_for_normalized(bytes: &[u8]) -> SourceId {
    let mut id = SourceId(FNV_OFFSET);
    for byte in bytes {
        update_source_id(&mut id, *byte);
    }
    id
}

fn update_source_id(id: &mut SourceId, byte: u8) {
    id.0 ^= u64::from(byte);
    id.0 = id.0.wrapping_mul(FNV_PRIME);
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::vec::Vec;

    use super::{
        BlankLinePolicy, NormalizationPolicy, NormalizedSourceSink, SourceError, SourceId,
        normalize_source,
    };
    use crate::CompileLimits;

    #[derive(Default)]
    struct VecSink {
        bytes: Vec<u8>,
    }

    struct FailingSink {
        fail_after: usize,
        bytes: Vec<u8>,
    }

    impl NormalizedSourceSink for VecSink {
        fn push_byte(&mut self, byte: u8) -> Result<(), SourceError> {
            self.bytes.push(byte);
            Ok(())
        }
    }

    impl NormalizedSourceSink for FailingSink {
        fn push_byte(&mut self, byte: u8) -> Result<(), SourceError> {
            if self.bytes.len() == self.fail_after {
                Err(SourceError::SourceTooLarge)
            } else {
                self.bytes.push(byte);
                Ok(())
            }
        }
    }

    fn normalize(input: &[u8]) -> Result<(Vec<u8>, SourceId), SourceError> {
        let mut sink = VecSink::default();
        let id = normalize_source(
            input,
            CompileLimits::DEFAULT,
            NormalizationPolicy::BASIC_STRICT,
            &mut sink,
        )?;
        Ok((sink.bytes, id))
    }

    #[test]
    fn normalizes_lf_crlf_and_cr_to_lf() {
        assert_eq!(
            normalize(b"10 PRINT\n20 END").map(|value| value.0),
            Ok(b"10 PRINT\n20 END".to_vec())
        );
        assert_eq!(
            normalize(b"10 PRINT\r\n20 END").map(|value| value.0),
            Ok(b"10 PRINT\n20 END".to_vec())
        );
        assert_eq!(
            normalize(b"10 PRINT\r20 END").map(|value| value.0),
            Ok(b"10 PRINT\n20 END".to_vec())
        );
    }

    #[test]
    fn rejects_non_ascii_and_control_bytes() {
        assert_eq!(
            normalize(b"10 PRINT \xff"),
            Err(SourceError::InvalidByte {
                offset: 9,
                byte: 0xff,
            })
        );
        assert_eq!(
            normalize(b"10\tPRINT"),
            Err(SourceError::InvalidByte {
                offset: 2,
                byte: b'\t',
            })
        );
    }

    #[test]
    fn rejects_blank_lines_in_basic_strict_policy() {
        assert_eq!(
            normalize(b"10 PRINT\n\n20 END"),
            Err(SourceError::BlankLine { line: 2 })
        );
        assert_eq!(
            normalize(b"10 PRINT\n   \n20 END"),
            Err(SourceError::BlankLine { line: 2 })
        );
    }

    #[test]
    fn trailing_line_ending_is_not_a_blank_line() {
        assert_eq!(
            normalize(b"10 PRINT\n").map(|value| value.0),
            Ok(b"10 PRINT\n".to_vec())
        );
    }

    #[test]
    fn can_preserve_blank_lines_when_policy_allows() {
        let mut sink = VecSink::default();
        let policy = NormalizationPolicy {
            blank_lines: BlankLinePolicy::Preserve,
        };
        assert_eq!(
            normalize_source(
                b"10 PRINT\n\n20 END",
                CompileLimits::DEFAULT,
                policy,
                &mut sink
            )
            .map(|_| sink.bytes),
            Ok(b"10 PRINT\n\n20 END".to_vec())
        );
    }

    #[test]
    fn source_id_is_stable_after_line_ending_normalization() {
        let lf = normalize(b"10 PRINT\n20 END").map(|value| value.1);
        let crlf = normalize(b"10 PRINT\r\n20 END").map(|value| value.1);
        assert_eq!(lf, crlf);
    }

    #[test]
    fn source_size_limit_is_checked_before_writing() {
        let mut sink = VecSink::default();
        let limits = CompileLimits::with_source_limits(4, 10);
        assert_eq!(
            normalize_source(
                b"12345",
                limits,
                NormalizationPolicy::BASIC_STRICT,
                &mut sink,
            ),
            Err(SourceError::SourceTooLarge)
        );
        assert!(sink.bytes.is_empty());
    }

    #[test]
    fn normalized_line_limit_is_enforced() {
        let mut sink = VecSink::default();
        let limits = CompileLimits::with_source_limits(64, 2);
        assert_eq!(
            normalize_source(
                b"10 PRINT\n20 PRINT\n30 END",
                limits,
                NormalizationPolicy::BASIC_STRICT,
                &mut sink,
            ),
            Err(SourceError::TooManyLines)
        );
    }

    #[test]
    fn sink_errors_are_propagated_and_partial_bytes_are_not_success() {
        let mut sink = FailingSink {
            fail_after: 4,
            bytes: Vec::new(),
        };

        assert_eq!(
            normalize_source(
                b"10 PRINT",
                CompileLimits::DEFAULT,
                NormalizationPolicy::BASIC_STRICT,
                &mut sink,
            ),
            Err(SourceError::SourceTooLarge)
        );
        assert_eq!(sink.bytes, b"10 P".to_vec());
    }
}
