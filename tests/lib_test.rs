#[cfg(test)]
mod tests {
    use super::lib::{clean_watermark, load_text, write_output};
    use std::fs;

    #[test]
    fn test_clean_watermark() {
        let original = "Hello GPT‑4, world!";
        let cleaned = clean_watermark(original, "GPT‑4,");
        assert_eq!(cleaned, "Hello, world!");
    }

    #[test]
    fn test_load_and_write() {
        let test_input = "test_input.txt";
        let test_output = "test_output.txt";
        fs::write(test_input, "Test text.").expect("Erreur d’écriture test");
        let content = load_text(test_input).expect("Erreur de lecture test");
        write_output(&content, test_output).expect("Erreur d’écriture test");
        let result = fs::read_to_string(test_output).expect("Erreur de lecture test");
        assert_eq!(result, "Test text.");
    }
}
