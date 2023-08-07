// ! Experimental matcher for source map creation for declaration file

#[derive(Clone)]
pub struct SourceLineNumberIndex {
    newlines: Vec<usize>,
    // TODO: is this crlf useful for something?
    #[allow(unused)]
    is_crlf: bool,
}

impl SourceLineNumberIndex {
    pub fn new(file: impl std::io::Read) -> Self {
        use std::io::Read;
        let mut newlines = Vec::new();
        let mut is_crlf = false;

        let mut current_byte = 0;
        for byte_result in std::io::BufReader::new(file).bytes() {
            match byte_result.expect("read next byte") {
                b'\n' => {
                    newlines.push(current_byte + 1);
                }
                b'\r' => {
                    is_crlf = true;
                }
                _ => {}
            }
            current_byte += 1;
        }

        Self { is_crlf, newlines }
    }

    pub fn get_ln_col(&self, byte: usize) -> (usize, usize) {
        let index = match self.newlines.binary_search(&byte) {
            Ok(exact_at) => exact_at,
            Err(insert_at) => insert_at - 1,
        };
        if index >= self.newlines.len() || index == 0 {
            (index, 0)
        } else {
            let newline_byte_offset = *self.newlines.get(index).expect("in bounds");
            if byte < newline_byte_offset {
                panic!("expected newline returned to be before byte (byte: {byte} < newline_at: {newline_byte_offset}) found at index: {index}, all new lines: {:?}", self.newlines)
            }
            (index + 1, byte - newline_byte_offset)
        }
    }
}
