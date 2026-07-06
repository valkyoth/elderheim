#![no_std]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatingSystem {
    Linux,
    Windows,
    MacOs,
    Bsd,
    Android,
    Ios,
    Aesynx,
}

impl OperatingSystem {
    pub const fn cli_name(self) -> &'static str {
        match self {
            Self::Linux => "linux",
            Self::Windows => "windows",
            Self::MacOs => "macos",
            Self::Bsd => "bsd",
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Aesynx => "aesynx",
        }
    }

    pub fn parse_cli_name(value: &str) -> Option<Self> {
        match value {
            "linux" => Some(Self::Linux),
            "windows" => Some(Self::Windows),
            "macos" => Some(Self::MacOs),
            "bsd" => Some(Self::Bsd),
            "android" => Some(Self::Android),
            "ios" => Some(Self::Ios),
            "aesynx" => Some(Self::Aesynx),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Architecture {
    X86,
    X86_64,
    Aarch32,
    Aarch64,
}

impl Architecture {
    pub const fn cli_name(self) -> &'static str {
        match self {
            Self::X86 => "x86",
            Self::X86_64 => "x86_64",
            Self::Aarch32 => "aarch32",
            Self::Aarch64 => "aarch64",
        }
    }

    pub fn parse_cli_name(value: &str) -> Option<Self> {
        match value {
            "x86" => Some(Self::X86),
            "x86_64" => Some(Self::X86_64),
            "aarch32" => Some(Self::Aarch32),
            "aarch64" => Some(Self::Aarch64),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutableFormat {
    Elf32,
    Elf64,
    Pe32,
    Pe64,
    MachO64,
    AesynxRaw,
}

impl ExecutableFormat {
    pub const fn cli_name(self) -> &'static str {
        match self {
            Self::Elf32 => "elf32",
            Self::Elf64 => "elf64",
            Self::Pe32 => "pe32",
            Self::Pe64 => "pe64",
            Self::MachO64 => "macho64",
            Self::AesynxRaw => "aesynxraw",
        }
    }

    pub fn parse_cli_name(value: &str) -> Option<Self> {
        match value {
            "elf32" => Some(Self::Elf32),
            "elf64" => Some(Self::Elf64),
            "pe32" => Some(Self::Pe32),
            "pe64" => Some(Self::Pe64),
            "macho64" => Some(Self::MachO64),
            "aesynxraw" => Some(Self::AesynxRaw),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TargetSpec {
    pub arch: Architecture,
    pub os: OperatingSystem,
    pub format: ExecutableFormat,
}

impl TargetSpec {
    pub const LINUX_X86_ELF32: Self = Self {
        arch: Architecture::X86,
        os: OperatingSystem::Linux,
        format: ExecutableFormat::Elf32,
    };

    pub const LINUX_X86_64_ELF64: Self = Self {
        arch: Architecture::X86_64,
        os: OperatingSystem::Linux,
        format: ExecutableFormat::Elf64,
    };

    pub const LINUX_AARCH32_ELF32: Self = Self {
        arch: Architecture::Aarch32,
        os: OperatingSystem::Linux,
        format: ExecutableFormat::Elf32,
    };

    pub const LINUX_AARCH64_ELF64: Self = Self {
        arch: Architecture::Aarch64,
        os: OperatingSystem::Linux,
        format: ExecutableFormat::Elf64,
    };

    pub const WINDOWS_X86_64_PE64: Self = Self {
        arch: Architecture::X86_64,
        os: OperatingSystem::Windows,
        format: ExecutableFormat::Pe64,
    };

    pub const MACOS_AARCH64_MACHO64: Self = Self {
        arch: Architecture::Aarch64,
        os: OperatingSystem::MacOs,
        format: ExecutableFormat::MachO64,
    };

    pub const SUPPORTED_1_0: [Self; 6] = [
        Self::LINUX_X86_ELF32,
        Self::LINUX_X86_64_ELF64,
        Self::LINUX_AARCH32_ELF32,
        Self::LINUX_AARCH64_ELF64,
        Self::WINDOWS_X86_64_PE64,
        Self::MACOS_AARCH64_MACHO64,
    ];

    pub const fn cli_name(self) -> Option<&'static str> {
        match (self.os, self.arch, self.format) {
            (OperatingSystem::Linux, Architecture::X86, ExecutableFormat::Elf32) => {
                Some("linux-x86-elf32")
            }
            (OperatingSystem::Linux, Architecture::X86_64, ExecutableFormat::Elf64) => {
                Some("linux-x86_64-elf64")
            }
            (OperatingSystem::Linux, Architecture::Aarch32, ExecutableFormat::Elf32) => {
                Some("linux-aarch32-elf32")
            }
            (OperatingSystem::Linux, Architecture::Aarch64, ExecutableFormat::Elf64) => {
                Some("linux-aarch64-elf64")
            }
            (OperatingSystem::Windows, Architecture::X86_64, ExecutableFormat::Pe64) => {
                Some("windows-x86_64-pe64")
            }
            (OperatingSystem::MacOs, Architecture::Aarch64, ExecutableFormat::MachO64) => {
                Some("macos-aarch64-macho64")
            }
            _ => None,
        }
    }

    pub fn parse_cli_name(value: &str) -> Result<Self, TargetParseError> {
        if value.is_empty() {
            return Err(TargetParseError::Empty);
        }

        let (os_value, remainder) = split_component(value).ok_or(TargetParseError::InvalidShape)?;
        let (arch_value, format_value) =
            split_component(remainder).ok_or(TargetParseError::InvalidShape)?;

        if format_value.contains('-') {
            return Err(TargetParseError::InvalidShape);
        }

        let os = OperatingSystem::parse_cli_name(os_value)
            .ok_or(TargetParseError::UnknownOperatingSystem)?;
        let arch = Architecture::parse_cli_name(arch_value)
            .ok_or(TargetParseError::UnknownArchitecture)?;
        let format = ExecutableFormat::parse_cli_name(format_value)
            .ok_or(TargetParseError::UnknownExecutableFormat)?;
        let target = Self { arch, os, format };

        match target.cli_name() {
            Some(_) => Ok(target),
            None => Err(TargetParseError::UnsupportedCombination),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TargetParseError {
    Empty,
    InvalidShape,
    UnknownOperatingSystem,
    UnknownArchitecture,
    UnknownExecutableFormat,
    UnsupportedCombination,
}

impl TargetParseError {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Empty => "E-TARGET-EMPTY",
            Self::InvalidShape => "E-TARGET-SHAPE",
            Self::UnknownOperatingSystem => "E-TARGET-OS",
            Self::UnknownArchitecture => "E-TARGET-ARCH",
            Self::UnknownExecutableFormat => "E-TARGET-FORMAT",
            Self::UnsupportedCombination => "E-TARGET-UNSUPPORTED",
        }
    }

    pub const fn message(self) -> &'static str {
        match self {
            Self::Empty => "target name is empty",
            Self::InvalidShape => "target name must use os-architecture-format",
            Self::UnknownOperatingSystem => "target operating system is not recognized",
            Self::UnknownArchitecture => "target architecture is not recognized",
            Self::UnknownExecutableFormat => "target executable format is not recognized",
            Self::UnsupportedCombination => "target combination is not supported for 1.0",
        }
    }
}

fn split_component(value: &str) -> Option<(&str, &str)> {
    if value.is_empty() {
        return None;
    }

    let (first, second) = value.split_once('-')?;

    if first.is_empty() || second.is_empty() {
        return None;
    }

    Some((first, second))
}

#[cfg(test)]
mod tests {
    use super::{Architecture, ExecutableFormat, OperatingSystem, TargetParseError, TargetSpec};

    #[test]
    fn first_target_is_elf64() {
        assert_eq!(
            TargetSpec::LINUX_X86_64_ELF64.format,
            ExecutableFormat::Elf64
        );
    }

    #[test]
    fn linux_targets_cover_32_and_64_bit_x86_and_aarch() {
        let targets = TargetSpec::SUPPORTED_1_0;
        let arches = targets.map(|target| target.arch);
        assert!(arches.contains(&Architecture::X86));
        assert!(arches.contains(&Architecture::X86_64));
        assert!(arches.contains(&Architecture::Aarch32));
        assert!(arches.contains(&Architecture::Aarch64));
    }

    #[test]
    fn desktop_targets_cover_windows_and_apple_silicon_macos() {
        assert_eq!(TargetSpec::WINDOWS_X86_64_PE64.os, OperatingSystem::Windows);
        assert_eq!(TargetSpec::WINDOWS_X86_64_PE64.arch, Architecture::X86_64);
        assert_eq!(
            TargetSpec::WINDOWS_X86_64_PE64.format,
            ExecutableFormat::Pe64
        );

        assert_eq!(TargetSpec::MACOS_AARCH64_MACHO64.os, OperatingSystem::MacOs);
        assert_eq!(
            TargetSpec::MACOS_AARCH64_MACHO64.arch,
            Architecture::Aarch64
        );
        assert_eq!(
            TargetSpec::MACOS_AARCH64_MACHO64.format,
            ExecutableFormat::MachO64
        );
    }

    #[test]
    fn one_zero_target_names_are_cli_visible() {
        let names = TargetSpec::SUPPORTED_1_0.map(TargetSpec::cli_name);
        assert_eq!(
            names,
            [
                Some("linux-x86-elf32"),
                Some("linux-x86_64-elf64"),
                Some("linux-aarch32-elf32"),
                Some("linux-aarch64-elf64"),
                Some("windows-x86_64-pe64"),
                Some("macos-aarch64-macho64"),
            ]
        );
    }

    #[test]
    fn supported_cli_names_round_trip() {
        for target in TargetSpec::SUPPORTED_1_0 {
            let name = target.cli_name().unwrap_or("missing-name");
            assert_eq!(TargetSpec::parse_cli_name(name), Ok(target));
        }
    }

    #[test]
    fn unsupported_target_strings_fail_with_stable_diagnostics() {
        let cases = [
            ("", TargetParseError::Empty),
            ("linux-x86", TargetParseError::InvalidShape),
            ("linux-x86-elf32-extra", TargetParseError::InvalidShape),
            (
                "solaris-x86-elf32",
                TargetParseError::UnknownOperatingSystem,
            ),
            ("linux-riscv64-elf64", TargetParseError::UnknownArchitecture),
            (
                "linux-x86_64-coff",
                TargetParseError::UnknownExecutableFormat,
            ),
            ("linux-x86-pe64", TargetParseError::UnsupportedCombination),
            (
                "windows-aarch64-pe64",
                TargetParseError::UnsupportedCombination,
            ),
            (
                "macos-aarch64-elf64",
                TargetParseError::UnsupportedCombination,
            ),
        ];

        for (input, expected) in cases {
            let actual = TargetSpec::parse_cli_name(input);
            assert_eq!(actual, Err(expected));
            assert!(!expected.code().is_empty());
            assert!(!expected.message().is_empty());
        }
    }
}
