use core::fmt::{self, Write};

use crate::{Diagnostic, PipelineStage, StageOutcome};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReportSection {
    Summary,
    Pipeline,
    Diagnostics,
}

impl ReportSection {
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::Pipeline => "pipeline",
            Self::Diagnostics => "diagnostics",
        }
    }

    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::Summary => "Summary",
            Self::Pipeline => "Pipeline",
            Self::Diagnostics => "Diagnostics",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReportEvent {
    Section(ReportSection),
    StageStarted(PipelineStage),
    StageFinished {
        stage: PipelineStage,
        outcome: StageOutcome,
    },
    Diagnostic {
        stage: PipelineStage,
        diagnostic: Diagnostic,
    },
}

impl ReportEvent {
    pub fn render<W: Write>(self, writer: &mut W) -> fmt::Result {
        match self {
            Self::Section(section) => render_section(section, writer),
            Self::StageStarted(stage) => {
                writeln!(
                    writer,
                    "stage-started {} {}",
                    stage.ordinal(),
                    stage.label()
                )
            }
            Self::StageFinished { stage, outcome } => writeln!(
                writer,
                "stage-finished {} {} {}",
                stage.ordinal(),
                stage.label(),
                outcome.label()
            ),
            Self::Diagnostic { stage, diagnostic } => writeln!(
                writer,
                "diagnostic {} {} {} {}",
                stage.label(),
                diagnostic.severity.label(),
                diagnostic.code.code(),
                diagnostic.code.message()
            ),
        }
    }
}

pub fn render_section<W: Write>(section: ReportSection, writer: &mut W) -> fmt::Result {
    writeln!(writer, "[{}]", section.key())?;
    writeln!(writer, "title: {}", section.title())
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::string::String;

    use super::{ReportEvent, ReportSection};
    use crate::{Diagnostic, DiagnosticCode, PipelineStage, Span, StageOutcome};

    #[test]
    fn report_section_rendering_is_golden() {
        let mut rendered = String::new();
        assert_eq!(
            ReportEvent::Section(ReportSection::Pipeline).render(&mut rendered),
            Ok(())
        );
        assert_eq!(rendered, "[pipeline]\ntitle: Pipeline\n");
    }

    #[test]
    fn report_stage_events_are_golden() {
        let mut rendered = String::new();
        assert_eq!(
            ReportEvent::StageStarted(PipelineStage::SourceToDiagnostic).render(&mut rendered),
            Ok(())
        );
        assert_eq!(
            ReportEvent::StageFinished {
                stage: PipelineStage::SourceToDiagnostic,
                outcome: StageOutcome::Complete,
            }
            .render(&mut rendered),
            Ok(())
        );
        assert_eq!(
            rendered,
            "stage-started 0 source-to-diagnostic\nstage-finished 0 source-to-diagnostic complete\n"
        );
    }

    #[test]
    fn report_diagnostic_event_is_golden() {
        let mut rendered = String::new();
        let diagnostic = Diagnostic::error(DiagnosticCode::InvalidDialect, Span::point(0));
        assert_eq!(
            ReportEvent::Diagnostic {
                stage: PipelineStage::SourceToDiagnostic,
                diagnostic,
            }
            .render(&mut rendered),
            Ok(())
        );
        assert_eq!(
            rendered,
            "diagnostic source-to-diagnostic error E-CORE-INVALID-DIALECT selected language dialect is not recognized\n"
        );
    }
}
