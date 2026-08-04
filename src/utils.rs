use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub enum IndentStyle {
    #[serde(rename = "tab")]
    Tab,
    #[serde(rename = "space")]
    #[default]
    Space,
}

impl From<IndentStyle> for char {
    fn from(val: IndentStyle) -> Self {
        match val {
            IndentStyle::Tab => '\t',
            IndentStyle::Space => ' ',
        }
    }
}

pub trait DeIndent {
    fn de_indent(&self, indent_style: IndentStyle) -> Self;
}

impl DeIndent for String {
    fn de_indent(&self, indent_style: IndentStyle) -> Self {
        let indentation = get_indentation(self, indent_style);

        let mut input = self.lines().peekable();

        let mut de_indented_input = String::new();

        while let Some(line) = input.next() {
            if line.trim_start().is_empty()
                && (de_indented_input.is_empty() || input.peek().is_none())
            {
                continue;
            }
            if !de_indented_input.is_empty() {
                de_indented_input.push('\n');
            }
            de_indented_input.push_str(&trim_start_until(line, indentation, indent_style))
        }
        de_indented_input
    }
}

/// Get the indentation number of a string.
/// This implementation ignores tabs.
pub fn get_indentation(input: &str, indent_style: IndentStyle) -> usize {
    if input.is_empty() {
        return 0;
    }
    let indentation = input
        .lines()
        .fold(None, |smallest_indentation, line: &str| {
            if let Some(indentation) = get_line_indentation(line, indent_style) {
                let Some(smallest_indentation) = smallest_indentation else {
                    return Some(indentation);
                };
                if indentation < smallest_indentation {
                    return Some(indentation);
                } else {
                    return Some(smallest_indentation);
                }
            }
            smallest_indentation
        });
    indentation.unwrap_or(0)
}

#[cfg(test)]
mod get_indentation_tests {
    use super::*;

    #[test]
    fn test_get_indentation_empty_input() {
        let input = "";
        assert_eq!(get_indentation(input, IndentStyle::Space), 0);
    }

    #[test]
    fn test_get_indentation_single_line_no_indentation() {
        let input = "hello world";
        assert_eq!(get_indentation(input, IndentStyle::Space), 0);
    }

    #[test]
    fn test_get_indentation_single_line_with_indentation() {
        let input = "    hello world";
        assert_eq!(get_indentation(input, IndentStyle::Space), 4);
    }

    #[test]
    fn test_get_indentation_multiple_lines_no_indentation() {
        let input = "hello world\nhow are you?";
        assert_eq!(get_indentation(input, IndentStyle::Space), 0);
    }

    #[test]
    fn test_get_indentation_multiple_lines_with_indentation() {
        let input = "     hello world\n    how are you?";
        assert_eq!(get_indentation(input, IndentStyle::Space), 4);
    }

    #[test]
    fn test_get_indentation_mixed_indentation() {
        let input = "    hello world\nhow are you?";
        assert_eq!(get_indentation(input, IndentStyle::Space), 0);
    }

    #[test]
    fn test_get_indentation_only_indentation() {
        let input = "    \n    \n    ";
        assert_eq!(get_indentation(input, IndentStyle::Space), 0);
    }
}

/// Get the indentation number of a line.
/// a line is supposed to be a string without a newline character, or with a trailing newline character.
/// This implementation ignores tabs.
pub fn get_line_indentation(line: &str, indent_style: IndentStyle) -> Option<usize> {
    if line.is_empty() {
        return None;
    }
    let indent_style: char = indent_style.into();
    let mut indentation = 0;
    for char in line.chars() {
        if char == indent_style {
            indentation += 1;
        } else {
            break;
        }
    }
    if indentation == line.len() {
        None
    } else {
        Some(indentation)
    }
}

#[cfg(test)]
mod get_line_indentation_tests {
    use super::*;

    #[test]
    fn test_get_line_indentation_empty_line() {
        let line = "";
        assert_eq!(get_line_indentation(line, IndentStyle::Space), None);
    }

    #[test]
    fn test_get_line_indentation_only_spaces() {
        let line = "    ";
        assert_eq!(get_line_indentation(line, IndentStyle::Space), None);
    }

    #[test]
    fn test_get_line_indentation_no_indentation() {
        let line = "hello world";
        assert_eq!(get_line_indentation(line, IndentStyle::Space), Some(0));
    }

    #[test]
    fn test_get_line_indentation_with_text() {
        let line = "    hello world";
        assert_eq!(get_line_indentation(line, IndentStyle::Space), Some(4));
    }

    #[test]
    fn test_get_line_indentation_with_trailing_newline() {
        let line = "    hello world\n";
        assert_eq!(get_line_indentation(line, IndentStyle::Space), Some(4));
    }
    #[test]
    fn test_get_line_indentation_with_tab() {
        assert_eq!(
            get_line_indentation("\t\thello world\n", IndentStyle::Tab),
            Some(2)
        );
    }
}

/// Trim the start of a string until a certain amount of characters have been trimmed.
/// It stops trimming when it encounters a non-whitespace character.
pub fn trim_start_until(input: &str, amount: usize, indent_style: IndentStyle) -> String {
    let mut result = String::new();
    let mut trimmed = 0;

    if input == "\n" {
        return input.to_string();
    }
    let indent_style: char = indent_style.into();

    for (index, char) in input.char_indices() {
        if char != indent_style {
            let (_, rest) = input.split_at(index);
            result.push_str(rest);
            return result;
        }

        if trimmed < amount {
            trimmed += 1;
        } else {
            result.push(char);
        }
    }
    result
}

#[cfg(test)]
mod trim_start_until_tests {
    use super::*;

    #[test]
    fn test_trim_start_until_empty_input() {
        let input = "";
        let amount = 5;
        let expected_output = "";
        assert_eq!(
            trim_start_until(input, amount, IndentStyle::Space),
            expected_output
        );
    }

    #[test]
    fn test_trim_start_until_no_trim_needed() {
        let input = "hello world";
        let amount = 5;
        let expected_output = "hello world";
        assert_eq!(
            trim_start_until(input, amount, IndentStyle::Space),
            expected_output
        );
    }

    #[test]
    fn test_trim_start_until_trimmed_all() {
        let input = "     ";
        let amount = 5;
        let expected_output = "";
        assert_eq!(
            trim_start_until(input, amount, IndentStyle::Space),
            expected_output
        );
    }

    #[test]
    fn test_trim_start_until_trimmed_some() {
        let input = "     hello world";
        let amount = 5;
        let expected_output = "hello world";
        assert_eq!(
            trim_start_until(input, amount, IndentStyle::Space),
            expected_output
        );
    }

    #[test]
    fn test_trim_start_until_trimmed_until_non_whitespace() {
        let input = "     hello     world";
        let amount = 5;
        let expected_output = "hello     world";
        assert_eq!(
            trim_start_until(input, amount, IndentStyle::Space),
            expected_output
        );
    }

    #[test]
    fn test_trim_start_until_newline_input() {
        let input = "\n";
        let amount = 5;
        let expected_output = "\n";
        assert_eq!(
            trim_start_until(input, amount, IndentStyle::Space),
            expected_output
        );
    }

    #[test]
    fn test_trim_start_tab() {
        assert_eq!(trim_start_until("\t\t", 2, IndentStyle::Tab), "");
        assert_eq!(trim_start_until("\t\t", 3, IndentStyle::Tab), "");
        assert_eq!(trim_start_until("\t\t", 1, IndentStyle::Tab), "\t");
        assert_eq!(trim_start_until("\t\t", 0, IndentStyle::Tab), "\t\t");
        assert_eq!(trim_start_until(" \t\t", 1, IndentStyle::Tab), " \t\t");
    }
}

#[cfg(test)]
mod lines_tests {

    #[test]
    fn test_lines() {
        let input = "\n1\n2\n\n3\n\n";
        let lines = input.lines().collect::<Vec<&str>>();
        assert_eq!(lines.len(), 6);
    }
}
