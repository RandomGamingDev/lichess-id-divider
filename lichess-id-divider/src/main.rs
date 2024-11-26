use std::fs::File;
use std::io::prelude::*;
use base64::prelude::*;

fn main() -> std::io::Result<()> {
    let mut reader = my_reader::BufReader::open("../lichess_db_standard_rated_2024-10.pgn")?;
    let mut buffer = String::new();

    let mut file_cursor: usize = 0;
    let mut write_cursor: usize = 0;
    let mut to_write : [u8; 6 * 1000] = [0u8;6 * 1000];
    while let Some(line) = reader.read_line(&mut buffer) {
        let line_str: &str = line?.trim();
        if line_str.starts_with("[Site ") {
            let id: &str = &line_str[27..27 + 8];
            to_write[write_cursor..write_cursor + 6].copy_from_slice(&BASE64_STANDARD.decode(id).unwrap());
            write_cursor += 6;
            if write_cursor >= to_write.len() {
                write_cursor = 0;
                let mut writer = File::create(format!("../test/lichess-{file_cursor}.ids"))?;
                file_cursor += 1;
                writer.write_all(&to_write)?;
            }
        }
    }

    Ok(())
}

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}
