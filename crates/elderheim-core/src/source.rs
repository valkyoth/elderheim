mod location;
mod normalize;

pub use location::{LineColumn, LineCursor, Source, SourceError, SourceSpanLines};
pub use normalize::{
    BlankLinePolicy, NormalizationPolicy, NormalizedSourceSink, SourceId, normalize_source,
};
