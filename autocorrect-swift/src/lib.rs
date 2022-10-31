use autocorrect::LintResult;

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge::bridge(swift_repr = "struct")]
    pub struct LintResult {
        raw: String,
        filepath: String,
        lines: Vec<LineResult>,
        error: String,
    }

    #[swift_bridge::bridge(swift_repr = "struct")]
    pub struct LineResult {
        old: String,
        new: String,
        line: usize,
        col: usize,
        severity: usize,
    }

    extern "Swift" {
        type AutoCorrect;

        fn format(input: &str) -> String;
        fn format_for(input: &str, filepath: &str) -> String;
        fn lint_for(input: &str, filepath: &str) -> LintResult;
    }
}

fn format(input: &str) -> String {
    autocorrect::format(input)
}

fn format_for(input: &str, filepath: &str) -> String {
    autocorrect::format_for(input, filepath).out
}

fn lint_for(input: &str, filepath: &str) -> ffi::LintResult {
    let result = autocorrect::lint_for(input, filepath);

    ffi::LintResult {
        raw: result.raw.clone(),
        filepath: result.filepath.clone(),
        lines: result
            .lines
            .iter()
            .map(|l| ffi::LineResult {
                old: l.old.clone(),
                new: l.new.clone(),
                line: l.line,
                col: l.col,
                severity: l.severity as usize,
            })
            .collect(),
        error: result.error.clone(),
    }
}
