#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DartmouthBasicVersion {
    Version1,
    Version2,
    Version4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dialect {
    pub version: DartmouthBasicVersion,
    pub requires_line_numbers: bool,
}

impl Dialect {
    pub const V1: Self = Self {
        version: DartmouthBasicVersion::Version1,
        requires_line_numbers: true,
    };

    pub const V2: Self = Self {
        version: DartmouthBasicVersion::Version2,
        requires_line_numbers: true,
    };

    pub const V4: Self = Self {
        version: DartmouthBasicVersion::Version4,
        requires_line_numbers: true,
    };
}

#[cfg(test)]
mod tests {
    use super::Dialect;

    fn requires_line_numbers(dialect: Dialect) -> bool {
        dialect.requires_line_numbers
    }

    #[test]
    fn dartmouth_versions_require_line_numbers() {
        assert!(requires_line_numbers(Dialect::V1));
        assert!(requires_line_numbers(Dialect::V2));
        assert!(requires_line_numbers(Dialect::V4));
    }
}
