use std::time::{Duration, Instant};

pub struct TokenBucket {
    capacity: u64,
    tokens: u64,
    refill_rate: u64,
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(rate: u64) -> Self {
        Self {
            capacity: rate,
            tokens: rate,
            refill_rate: rate,
            last_refill: Instant::now(),
        }
    }

    fn refill(&mut self) {
        let elapsed = self.last_refill.elapsed().as_secs_f64();
        let new_tokens = (elapsed * self.refill_rate as f64) as u64;

        if new_tokens > 0 {
            self.tokens = (self.tokens + new_tokens).min(self.capacity);
            self.last_refill = Instant::now();
        }
    }

    pub async fn acquire(&mut self) {
        loop {
            self.refill();

            if self.tokens > 0 {
                self.tokens -= 1;
                return;
            }

            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
}
