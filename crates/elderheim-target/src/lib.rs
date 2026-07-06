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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Architecture {
    X86,
    X86_64,
    Aarch32,
    Aarch64,
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
}

#[cfg(test)]
mod tests {
    use super::{Architecture, ExecutableFormat, OperatingSystem, TargetSpec};

    #[test]
    fn first_target_is_elf64() {
        assert_eq!(
            TargetSpec::LINUX_X86_64_ELF64.format,
            ExecutableFormat::Elf64
        );
    }

    #[test]
    fn linux_targets_cover_32_and_64_bit_x86_and_aarch() {
        let targets = [
            TargetSpec::LINUX_X86_ELF32,
            TargetSpec::LINUX_X86_64_ELF64,
            TargetSpec::LINUX_AARCH32_ELF32,
            TargetSpec::LINUX_AARCH64_ELF64,
        ];
        let arches = targets.map(|target| target.arch);
        assert_eq!(
            arches,
            [
                Architecture::X86,
                Architecture::X86_64,
                Architecture::Aarch32,
                Architecture::Aarch64
            ]
        );
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
}
