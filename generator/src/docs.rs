fn is_list_item(line: &str) -> bool {
    line.starts_with("- ")
        || line.starts_with("* ")
        || line
            .split_once(". ")
            .map(|(prefix, _)| prefix.chars().all(|c| c.is_ascii_digit()))
            .unwrap_or(false)
}

fn strip_leading_breaks(mut line: &str) -> &str {
    loop {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("<br>") {
            line = rest;
        } else if let Some(rest) = trimmed.strip_prefix("<br/>") {
            line = rest;
        } else if let Some(rest) = trimmed.strip_prefix("</br>") {
            line = rest;
        } else {
            return trimmed;
        }
    }
}

fn is_section_label(line: &str) -> bool {
    strip_leading_breaks(line).starts_with("**")
}

fn normalized_doc_lines(description: &str) -> Vec<String> {
    let lines = description.lines().collect::<Vec<_>>();

    let start = lines.iter().position(|line| !line.trim().is_empty());
    let end = lines.iter().rposition(|line| !line.trim().is_empty());
    let Some(start) = start else {
        return Vec::new();
    };
    let end = end.unwrap();

    let mut rendered = Vec::new();
    let mut previous_was_list_item = false;
    let mut in_list_continuation = false;
    let mut in_code_block = false;

    for line in &lines[start..=end] {
        let raw = line.trim_end_matches('\r').trim_end();
        let trimmed = strip_leading_breaks(raw.trim()).trim_end();

        if in_code_block {
            rendered.push(raw.to_string());
            if trimmed.starts_with("```") {
                in_code_block = false;
            }
            continue;
        }

        if trimmed.is_empty() {
            rendered.push(String::new());
            previous_was_list_item = false;
            in_list_continuation = false;
            continue;
        }

        let is_list_item = is_list_item(trimmed);
        if !previous_was_list_item
            && !in_list_continuation
            && is_list_item
            && !rendered.last().is_none_or(|line: &String| line.is_empty())
        {
            rendered.push(String::new());
        }

        if (previous_was_list_item || in_list_continuation)
            && !is_list_item
            && is_section_label(trimmed)
        {
            if !rendered.last().is_none_or(|line: &String| line.is_empty()) {
                rendered.push(String::new());
            }
            rendered.push(trimmed.to_string());
            in_list_continuation = false;
        } else if (previous_was_list_item || in_list_continuation) && !is_list_item {
            rendered.push(format!("   {trimmed}"));
            in_list_continuation = true;
        } else {
            rendered.push(trimmed.to_string());
            in_list_continuation = false;
        }

        if trimmed.starts_with("```") {
            in_code_block = true;
        }
        previous_was_list_item = is_list_item;
    }

    while rendered.first().is_some_and(|line| line.is_empty()) {
        rendered.remove(0);
    }
    while rendered.last().is_some_and(|line| line.is_empty()) {
        rendered.pop();
    }

    let mut collapsed = Vec::new();
    for line in rendered {
        if line.is_empty()
            && collapsed
                .last()
                .is_some_and(|prev: &String| prev.is_empty())
        {
            continue;
        }
        collapsed.push(line);
    }

    collapsed
}

pub(crate) fn render_block_doc_lines(description: &str) -> String {
    let mut lines = normalized_doc_lines(description);
    if lines.is_empty() {
        return String::new();
    }

    let first = lines.remove(0);
    let mut rendered = first;
    for line in lines {
        if line.is_empty() {
            rendered.push_str("\n *");
        } else if line.starts_with(' ') {
            rendered.push_str("\n *  ");
            rendered.push_str(&line);
        } else {
            rendered.push_str("\n * ");
            rendered.push_str(&line);
        }
    }
    rendered
}

pub(crate) fn render_doc_text(description: &str) -> String {
    normalized_doc_lines(description).join("\n")
}

pub(crate) fn render_line_doc_comment(description: &str) -> String {
    normalized_doc_lines(description)
        .into_iter()
        .map(|line| {
            if line.is_empty() {
                "///".to_string()
            } else {
                format!("/// {line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{render_block_doc_lines, render_line_doc_comment};

    #[test]
    fn indents_list_continuations() {
        let docs = "EmailSettings consists of:\n\n* bccEmailAddresses - An array.\nDocuSign verifies the email.\n*Example*: example text.";

        assert_eq!(
            render_line_doc_comment(docs),
            "/// EmailSettings consists of:\n///\n/// * bccEmailAddresses - An array.\n///    DocuSign verifies the email.\n///    *Example*: example text."
        );
    }

    #[test]
    fn drops_empty_docs_and_trailing_whitespace() {
        assert_eq!(render_line_doc_comment(" \n<br>  \n"), "");
        assert_eq!(
            render_line_doc_comment("first line  \nsecond line\t"),
            "/// first line\n/// second line"
        );
    }

    #[test]
    fn renders_block_doc_lines() {
        assert_eq!(
            render_block_doc_lines("one\n\n- two\nthree"),
            "one\n *\n * - two\n *     three"
        );
    }

    #[test]
    fn renders_plain_doc_text() {
        assert_eq!(
            super::render_doc_text("one\n\n- two\nthree"),
            "one\n\n- two\n   three"
        );
    }
}
