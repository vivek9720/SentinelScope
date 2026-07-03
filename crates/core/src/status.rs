use crate::diag::{Diagnostic, DiagnosticSet, Severity};



#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Status { Clean, Suspicious, Invalid, Blocked }



impl Status {

    pub fn as_str(self) -> &'static str {

        match self { Status::Clean => "clean", Status::Suspicious => "suspicious", Status::Invalid => "invalid", Status::Blocked => "blocked" }

    }

    pub fn combine(self, other: Status) -> Status {

        use Status::*;

        match (self, other) {

            (Invalid, _) | (_, Invalid) => Invalid,

            (Blocked, _) | (_, Blocked) => Blocked,

            (Suspicious, _) | (_, Suspicious) => Suspicious,

            _ => Clean,

        }

    }

}



#[derive(Debug, Clone)]

pub struct ValidationReport {

    pub subject: String,

    pub status: Status,

    pub diagnostics: DiagnosticSet,

}



impl ValidationReport {

    pub fn new(subject: impl Into<String>) -> Self { Self { subject: subject.into(), status: Status::Clean, diagnostics: DiagnosticSet::new() } }

    pub fn add(&mut self, diagnostic: Diagnostic) {

        self.status = match diagnostic.severity {

            Severity::Info | Severity::Low => self.status.combine(Status::Suspicious),

            Severity::Medium | Severity::High | Severity::Critical => self.status.combine(Status::Invalid),

        };

        self.diagnostics.push(diagnostic);

    }

    pub fn mark_blocked(&mut self, message: impl Into<String>) { self.status = self.status.combine(Status::Blocked); self.diagnostics.push(Diagnostic::high(message)); }

    pub fn merge(&mut self, other: ValidationReport) { self.status = self.status.combine(other.status); self.diagnostics.extend(other.diagnostics.into_vec()); }

    pub fn ok(&self) -> bool { self.status == Status::Clean }

    pub fn summary(&self) -> String { format!("{}: {} ({} findings)", self.subject, self.status.as_str(), self.diagnostics.len()) }

}
