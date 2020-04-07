use std::time::{Duration, Instant};

pub fn timed<T>(body: impl FnOnce() -> T) -> (T, Duration) {
  let start = Instant::now();
  let result = body();
  let end = Instant::now();
  (result, end.duration_since(start))
}
