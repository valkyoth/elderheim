#![no_std]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeRequirement {
    StaticTextOutput,
    IntegerOutput,
    IntegerInput,
    ArrayBoundsChecks,
    DivisionByZeroChecks,
    ProgramExit,
}

macro_rules! runtime_fragments {
    ($(($variant:ident, $name:literal, $bit:literal)),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum RuntimeFragment {
            $($variant,)+
        }

        impl RuntimeFragment {
            pub const ALL: [Self; runtime_fragments!(@count $($variant),+)] = [
                $(Self::$variant,)+
            ];

            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)+
                }
            }

            const fn bit(self) -> u16 {
                match self {
                    $(Self::$variant => $bit,)+
                }
            }
        }
    };
    (@count $($variant:ident),+) => {
        <[()]>::len(&[$(runtime_fragments!(@unit $variant)),+])
    };
    (@unit $variant:ident) => {
        ()
    };
}

runtime_fragments! {
    (WriteStatic, "write_static", 0b000_0001),
    (PrintI64, "print_i64", 0b000_0010),
    (ReadLine, "read_line", 0b000_0100),
    (ParseI64, "parse_i64", 0b000_1000),
    (BoundsFail, "bounds_fail", 0b001_0000),
    (DivZeroFail, "div_zero_fail", 0b010_0000),
    (Exit, "exit", 0b100_0000),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FragmentSet {
    bits: u16,
}

impl FragmentSet {
    #[must_use]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    pub const fn insert(&mut self, fragment: RuntimeFragment) {
        self.bits |= fragment.bit();
    }

    #[must_use]
    pub const fn contains(self, fragment: RuntimeFragment) -> bool {
        self.bits & fragment.bit() != 0
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimePlan {
    fragments: FragmentSet,
}

impl RuntimePlan {
    #[must_use]
    pub const fn fragments(self) -> FragmentSet {
        self.fragments
    }

    #[must_use]
    pub const fn emits_executable_artifacts(self) -> bool {
        false
    }

    #[must_use]
    pub const fn inclusion_report(self) -> FragmentInclusionReport {
        FragmentInclusionReport {
            fragments: self.fragments,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FragmentInclusionReport {
    fragments: FragmentSet,
}

impl FragmentInclusionReport {
    #[must_use]
    pub const fn entry(self, fragment: RuntimeFragment) -> FragmentInclusion {
        FragmentInclusion {
            fragment,
            included: self.fragments.contains(fragment),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FragmentInclusion {
    pub fragment: RuntimeFragment,
    pub included: bool,
}

#[must_use]
pub fn select_runtime(requirements: &[RuntimeRequirement]) -> RuntimePlan {
    let mut fragments = FragmentSet::empty();

    for requirement in requirements {
        apply_requirement(*requirement, &mut fragments);
    }

    RuntimePlan { fragments }
}

const fn apply_requirement(requirement: RuntimeRequirement, fragments: &mut FragmentSet) {
    match requirement {
        RuntimeRequirement::StaticTextOutput => {
            fragments.insert(RuntimeFragment::WriteStatic);
        }
        RuntimeRequirement::IntegerOutput => {
            fragments.insert(RuntimeFragment::WriteStatic);
            fragments.insert(RuntimeFragment::PrintI64);
        }
        RuntimeRequirement::IntegerInput => {
            fragments.insert(RuntimeFragment::ReadLine);
            fragments.insert(RuntimeFragment::ParseI64);
        }
        RuntimeRequirement::ArrayBoundsChecks => {
            fragments.insert(RuntimeFragment::WriteStatic);
            fragments.insert(RuntimeFragment::BoundsFail);
            fragments.insert(RuntimeFragment::Exit);
        }
        RuntimeRequirement::DivisionByZeroChecks => {
            fragments.insert(RuntimeFragment::WriteStatic);
            fragments.insert(RuntimeFragment::DivZeroFail);
            fragments.insert(RuntimeFragment::Exit);
        }
        RuntimeRequirement::ProgramExit => {
            fragments.insert(RuntimeFragment::Exit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FragmentSet, RuntimeFragment, RuntimeRequirement, select_runtime};

    #[test]
    fn empty_requirements_select_no_fragments() {
        let plan = select_runtime(&[]);
        assert!(plan.fragments().is_empty());
        assert!(!plan.emits_executable_artifacts());
    }

    #[test]
    fn static_text_output_selects_write_static_only() {
        let plan = select_runtime(&[RuntimeRequirement::StaticTextOutput]);
        let fragments = plan.fragments();

        assert!(fragments.contains(RuntimeFragment::WriteStatic));
        assert!(!fragments.contains(RuntimeFragment::PrintI64));
        assert!(!fragments.contains(RuntimeFragment::Exit));
    }

    #[test]
    fn integer_output_selects_formatting_dependency() {
        let plan = select_runtime(&[RuntimeRequirement::IntegerOutput]);
        let fragments = plan.fragments();

        assert!(fragments.contains(RuntimeFragment::WriteStatic));
        assert!(fragments.contains(RuntimeFragment::PrintI64));
        assert!(!fragments.contains(RuntimeFragment::ParseI64));
    }

    #[test]
    fn integer_input_selects_read_and_parse_fragments() {
        let plan = select_runtime(&[RuntimeRequirement::IntegerInput]);
        let fragments = plan.fragments();

        assert!(fragments.contains(RuntimeFragment::ReadLine));
        assert!(fragments.contains(RuntimeFragment::ParseI64));
        assert!(!fragments.contains(RuntimeFragment::WriteStatic));
    }

    #[test]
    fn failure_paths_select_message_and_exit_fragments() {
        let plan = select_runtime(&[
            RuntimeRequirement::ArrayBoundsChecks,
            RuntimeRequirement::DivisionByZeroChecks,
        ]);
        let fragments = plan.fragments();

        assert!(fragments.contains(RuntimeFragment::WriteStatic));
        assert!(fragments.contains(RuntimeFragment::BoundsFail));
        assert!(fragments.contains(RuntimeFragment::DivZeroFail));
        assert!(fragments.contains(RuntimeFragment::Exit));
    }

    #[test]
    fn duplicate_requirements_are_idempotent() {
        let first = select_runtime(&[RuntimeRequirement::ProgramExit]);
        let second = select_runtime(&[
            RuntimeRequirement::ProgramExit,
            RuntimeRequirement::ProgramExit,
        ]);

        assert_eq!(first, second);
    }

    #[test]
    fn inclusion_report_marks_selected_fragments() {
        let report = select_runtime(&[RuntimeRequirement::IntegerOutput]).inclusion_report();

        assert_eq!(
            report.entry(RuntimeFragment::PrintI64),
            super::FragmentInclusion {
                fragment: RuntimeFragment::PrintI64,
                included: true,
            }
        );
        assert_eq!(
            report.entry(RuntimeFragment::ReadLine),
            super::FragmentInclusion {
                fragment: RuntimeFragment::ReadLine,
                included: false,
            }
        );
    }

    #[test]
    fn integer_input_has_no_unused_output_or_failure_fragments() {
        let fragments = select_runtime(&[RuntimeRequirement::IntegerInput]).fragments();

        for fragment in RuntimeFragment::ALL {
            let expected = matches!(
                fragment,
                RuntimeFragment::ReadLine | RuntimeFragment::ParseI64
            );
            assert_eq!(
                fragments.contains(fragment),
                expected,
                "{}",
                fragment.name()
            );
        }
    }

    #[test]
    fn fragment_names_are_stable() {
        assert_eq!(RuntimeFragment::ALL.len(), 7);
        assert_eq!(RuntimeFragment::WriteStatic.name(), "write_static");
        assert_eq!(RuntimeFragment::PrintI64.name(), "print_i64");
        assert_eq!(RuntimeFragment::ReadLine.name(), "read_line");
        assert_eq!(RuntimeFragment::ParseI64.name(), "parse_i64");
        assert_eq!(RuntimeFragment::BoundsFail.name(), "bounds_fail");
        assert_eq!(RuntimeFragment::DivZeroFail.name(), "div_zero_fail");
        assert_eq!(RuntimeFragment::Exit.name(), "exit");
    }

    #[test]
    fn fragment_set_starts_empty() {
        assert_eq!(FragmentSet::empty(), FragmentSet::default());
    }
}
