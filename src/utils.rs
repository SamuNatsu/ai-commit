use std::io::{self, Write};

use anyhow::Result;
use regex::Regex;

pub fn filter_diff<S1, S2>(diff: S1, filter: S2) -> Result<String>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    // Create filter regular expression
    let re = format!(r"^diff --git a\/(.*\/)?({})", filter.as_ref());
    let re = Regex::new(&re)?;

    // Filter lines
    let mut filter_enabled = false;
    let ret = diff
        .as_ref()
        .lines()
        .filter(|line| {
            if re.is_match(&line) {
                filter_enabled = true;
                return false;
            }
            if filter_enabled && line.starts_with("diff --git") {
                filter_enabled = false;
            }
            return !filter_enabled;
        })
        .fold(String::new(), |prev, cur| format!("{prev}\n{cur}"));

    // Return filtered diff
    Ok(if ret.is_empty() {
        ret
    } else {
        ret[1..].to_owned()
    })
}

pub fn confirm<S: AsRef<str>>(prompt: S) -> Result<bool> {
    // Print prompt
    print!(
        "{}",
        console::style(format!("{} (y/N): ", prompt.as_ref()))
            .bold()
            .bright()
            .yellow()
    );
    io::stdout().flush()?;

    // Read user input
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    // Return result
    Ok(buf.trim().to_lowercase() == "y")
}
