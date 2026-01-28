use miette::{SourceOffset, SourceSpan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SrcSpan {
    pub start: usize,
    pub end: usize,
}

impl From<SourceSpan> for SrcSpan {
    fn from(span: SourceSpan) -> Self {
        SrcSpan {
            start: span.offset(),
            end: span.offset() + span.len(),
        }
    }
}

impl From<SrcSpan> for SourceSpan {
    fn from(span: SrcSpan) -> Self {
        SourceSpan::new(SourceOffset::from(span.start), span.end - span.start)
    }
}

impl Into<SourceSpan> for &SrcSpan {
    fn into(self) -> SourceSpan {
        SourceSpan::new(SourceOffset::from(self.start), self.end - self.start)
    }
}

impl Into<SrcSpan> for &SourceSpan {
    fn into(self) -> SrcSpan {
        SrcSpan {
            start: self.offset(),
            end: self.offset() + self.len(),
        }
    }
}
