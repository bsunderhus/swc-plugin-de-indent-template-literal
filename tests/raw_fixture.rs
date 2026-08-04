use std::{fs, path::PathBuf};
use swc_plugin_de_indent_template_literal::*;

#[cfg(test)]
mod space_tests {
    use super::*;
    #[testing::fixture("tests/fixture/raw/space/**/input.txt")]
    fn fixture_space(input: PathBuf) {
        let input_str = fs::read_to_string(&input).expect("Error in reading the input file");
        let output_str = fs::read_to_string(input.parent().unwrap().join("output.txt"))
            .expect("Error in reading the output file");
        assert_eq!(input_str.de_indent(IndentStyle::Space), output_str);
    }
}

#[cfg(test)]
mod tab_tests {
    use super::*;
    #[testing::fixture("tests/fixture/raw/tab/**/input.txt")]
    fn fixture_tab(input: PathBuf) {
        let input_str = fs::read_to_string(&input).expect("Error in reading the input file");
        let output_str = fs::read_to_string(input.parent().unwrap().join("output.txt"))
            .expect("Error in reading the output file");
        assert_eq!(input_str.de_indent(IndentStyle::Tab), output_str);
    }
}
