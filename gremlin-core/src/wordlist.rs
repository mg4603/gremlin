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
}
