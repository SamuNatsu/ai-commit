use std::io::{self, Write};

use anyhow::Result;
use regex::Regex;

pub fn filter_diff<S1, S2>(diff: S1, filter: S2) -> Result<String>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let re = format!(r"^diff --git a\/(.*\/)?({})", filter.as_ref());
    let re = Regex::new(&re)?;

    let mut is_lock_file = false;
    let ret = diff
        .as_ref()
        .lines()
        .filter(|line| {
            if re.is_match(&line) {
                is_lock_file = true;
                return false;
            }
            if is_lock_file && line.starts_with("diff --git") {
                is_lock_file = false;
            }
            return !is_lock_file;
        })
        .fold(String::new(), |prev, cur| format!("{prev}\n{cur}"));

    Ok(if ret.is_empty() {
        ret
    } else {
        ret[1..].to_owned()
    })
}

pub fn confirm<S: AsRef<str>>(prompt: S) -> Result<bool> {
    print!(
        "{}",
        console::style(format!("{} (y/N): ", prompt.as_ref()))
            .bold()
            .bright()
            .yellow()
    );
    io::stdout().flush()?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    Ok(buf.trim().to_lowercase() == "y")
}
