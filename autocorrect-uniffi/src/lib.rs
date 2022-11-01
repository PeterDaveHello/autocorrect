uniffi_macros::include_scaffolding!("autocorrect");

pub struct LintResult {
    raw: String,
    filepath: String,
    error: String,
    lines: Vec<LineResult>,
}

pub struct LineResult {
    old: String,
    new: String,
    line: u64,
    col: u64,
    severity: u8,
}

fn format(text: String) -> String {
    autocorrect::format(&text)
}

fn format_for(text: String, filepath: String) -> String {
    autocorrect::format_for(&text, &filepath).out
}

fn lint_for(text: String, filepath: String) -> LintResult {
    let result = autocorrect::lint_for(&text, &filepath);
    LintResult {
        raw: result.raw,
        filepath: result.filepath,
        error: result.error,
        lines: result
            .lines
            .into_iter()
            .map(|line| LineResult {
                old: line.old,
                new: line.new,
                line: line.line as u64,
                col: line.col as u64,
                severity: line.severity as u8,
            })
            .collect(),
    }
}
