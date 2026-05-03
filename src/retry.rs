use std::time::Duration;

/// Exponential backoff retry policy for transient API failures.
#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub max_retries: usize,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 2,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(2),
        }
    }
}

impl RetryPolicy {
    pub fn backoff_for(&self, attempt: usize) -> Duration {
        let factor = 2u32.saturating_pow(attempt as u32);
        (self.initial_backoff * factor).min(self.max_backoff)
    }
}
