use log::{error, info, LevelFilter};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
  int verbosity = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
  let log_level = match verbosity

  let mut log_file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("log_file.txt")
    .unwrap()

  log::set_boxed_logger(Box::new(WriteLogger::new(log_level, log::LogConfig::default(), log_file))).unwrap();
  log::set_max_level(log_level);
}
