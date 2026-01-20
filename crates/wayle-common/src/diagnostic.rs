//! Pretty-printed diagnostic messages for user-facing errors and warnings.

use std::fmt::{self, Display, Formatter, Write as _};

use owo_colors::OwoColorize;

/// Diagnostic severity level.
#[derive(Debug, Clone, Copy)]
pub enum DiagnosticKind {
    Error,
    Warning,
}

/// A formatted diagnostic message with colored output.
///
/// Renders consistently styled error/warning messages with labeled fields
/// and optional hints. Colors auto-disable when output isn't a TTY.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    kind: DiagnosticKind,
    title: String,
    fields: Vec<(String, String)>,
    hint: Option<String>,
}

impl Diagnostic {
    /// Creates an error diagnostic.
    pub fn error(title: impl Into<String>) -> Self {
        Self {
            kind: DiagnosticKind::Error,
            title: title.into(),
            fields: Vec::new(),
            hint: None,
        }
    }

    /// Creates a warning diagnostic.
    pub fn warning(title: impl Into<String>) -> Self {
        Self {
            kind: DiagnosticKind::Warning,
            title: title.into(),
            fields: Vec::new(),
            hint: None,
        }
    }

    /// Adds a labeled field.
    pub fn field(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.push((label.into(), value.into()));
        self
    }

    /// Adds a hint/suggestion line.
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Prints the colored diagnostic directly to stderr.
    ///
    /// Use this instead of passing through tracing, which escapes ANSI codes.
    pub fn emit(&self) {
        eprintln!("{self}");
    }

    /// Returns plain text version suitable for log files.
    ///
    /// Same structure as `Display` but without ANSI color codes.
    pub fn to_plain(&self) -> String {
        let mut out = String::new();

        let label = match self.kind {
            DiagnosticKind::Error => "error:",
            DiagnosticKind::Warning => "warning:",
        };

        let _ = writeln!(out);
        let _ = writeln!(out, "{} {}", label, self.title);

        let max_label_len = self.fields.iter().map(|(l, _)| l.len()).max().unwrap_or(0);

        for (label, value) in &self.fields {
            let _ = writeln!(
                out,
                "    {:>width$}: {}",
                label,
                value,
                width = max_label_len
            );
        }

        if let Some(hint) = &self.hint {
            let _ = writeln!(out);
            let _ = writeln!(out, "    → {hint}");
        }

        out
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let label = match self.kind {
            DiagnosticKind::Error => "error:".red().bold().to_string(),
            DiagnosticKind::Warning => "warning:".yellow().bold().to_string(),
        };

        let title_styled = match self.kind {
            DiagnosticKind::Error => self.title.red().bold().to_string(),
            DiagnosticKind::Warning => self.title.yellow().bold().to_string(),
        };

        writeln!(f)?;
        writeln!(f, "{} {}", label, title_styled)?;

        let max_label_len = self.fields.iter().map(|(l, _)| l.len()).max().unwrap_or(0);

        for (label, value) in &self.fields {
            let padded_label = format!("{:>width$}", label, width = max_label_len);
            writeln!(f, "    {}: {}", padded_label.cyan(), value.white())?;
        }

        if let Some(hint) = &self.hint {
            writeln!(f)?;
            writeln!(f, "    {} {}", "→".green(), hint.green())?;
        }

        Ok(())
    }
}
