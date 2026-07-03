use std::fmt;



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

pub enum Severity { Info, Low, Medium, High, Critical }



impl Severity {

    pub fn as_str(self) -> &'static str {

        match self { Severity::Info => "info", Severity::Low => "low", Severity::Medium => "medium", Severity::High => "high", Severity::Critical => "critical" }

    }

    pub fn from_word(word: &str) -> Option<Self> {

        match word.trim().to_ascii_lowercase().as_str() {

            "info" | "informational" => Some(Severity::Info),

            "low" | "minor" => Some(Severity::Low),

            "medium" | "moderate" => Some(Severity::Medium),

            "high" | "major" => Some(Severity::High),

            "critical" | "crit" => Some(Severity::Critical),

            _ => None,

        }

    }

    pub fn max(self, other: Severity) -> Severity { if self >= other { self } else { other } }

}



#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Diagnostic {

    pub severity: Severity,

    pub message: String,

    pub offset: Option<usize>,

    pub context: Option<String>,

}



impl Diagnostic {

    pub fn new(severity: Severity, message: impl Into<String>) -> Self { Self { severity, message: message.into(), offset: None, context: None } }

    pub fn at(mut self, offset: usize) -> Self { self.offset = Some(offset); self }

    pub fn with_context(mut self, context: impl Into<String>) -> Self { self.context = Some(context.into()); self }

    pub fn info(message: impl Into<String>) -> Self { Self::new(Severity::Info, message) }

    pub fn low(message: impl Into<String>) -> Self { Self::new(Severity::Low, message) }

    pub fn medium(message: impl Into<String>) -> Self { Self::new(Severity::Medium, message) }

    pub fn high(message: impl Into<String>) -> Self { Self::new(Severity::High, message) }

    pub fn critical(message: impl Into<String>) -> Self { Self::new(Severity::Critical, message) }

}



impl fmt::Display for Diagnostic {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match (self.offset, self.context.as_ref()) {

            (Some(offset), Some(context)) => write!(f, "{} at {}: {} ({})", self.severity.as_str(), offset, self.message, context),

            (Some(offset), None) => write!(f, "{} at {}: {}", self.severity.as_str(), offset, self.message),

            (None, Some(context)) => write!(f, "{}: {} ({})", self.severity.as_str(), self.message, context),

            (None, None) => write!(f, "{}: {}", self.severity.as_str(), self.message),

        }

    }

}



impl std::error::Error for Diagnostic {}



pub type ParseResult<T> = Result<T, Diagnostic>;



#[derive(Debug, Clone, Default)]

pub struct DiagnosticSet { diagnostics: Vec<Diagnostic> }



impl DiagnosticSet {

    pub fn new() -> Self { Self { diagnostics: Vec::new() } }

    pub fn push(&mut self, diagnostic: Diagnostic) { self.diagnostics.push(diagnostic); }

    pub fn extend<I: IntoIterator<Item = Diagnostic>>(&mut self, items: I) { self.diagnostics.extend(items); }

    pub fn is_empty(&self) -> bool { self.diagnostics.is_empty() }

    pub fn len(&self) -> usize { self.diagnostics.len() }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> { self.diagnostics.iter() }

    pub fn into_vec(self) -> Vec<Diagnostic> { self.diagnostics }

    pub fn highest_severity(&self) -> Option<Severity> { self.diagnostics.iter().map(|d| d.severity).max() }

    pub fn has_at_least(&self, severity: Severity) -> bool { self.diagnostics.iter().any(|d| d.severity >= severity) }

    pub fn render_lines(&self) -> Vec<String> { self.diagnostics.iter().map(|d| d.to_string()).collect() }

}
