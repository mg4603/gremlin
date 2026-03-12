use std::path::Path;

use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader, Lines};

pub struct WordlistReader {
    lines: Lines<BufReader<File>>,
}

impl WordlistReader {
    pub async fn open(path: &Path) -> io::Result<Self> {
        let file = File::open(path).await?;
        let reader = BufReader::new(file);

        Ok(Self {
            lines: reader.lines(),
        })
    }

    pub async fn next(&mut self) -> io::Result<Option<String>> {
        self.lines.next_line().await
    }

    pub fn count_lines(path: &Path) -> io::Result<usize> {
        let file = std::fs::File::open(path)?;

        let mut reader = std::io::BufReader::new(file);
        let mut count = 0;
        let mut buf = [0u8; 65536];

        loop {
            let bytes_read = std::io::Read::read(&mut reader, &mut buf)?;
            if bytes_read == 0 {
                break;
            }

            count += buf[..bytes_read].iter().filter(|&&b| b == b'\n').count();
        }

        Ok(count)
    }
}
