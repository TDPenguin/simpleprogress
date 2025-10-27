use std::time::{Duration};
use std::thread::{sleep};
use simpleprogress::{BasicProgressBar, Spinner};

fn main() {
    // Test BasicProgressBar
    let mut basic_bar = BasicProgressBar::new(100.0)
        .no_arrow()
        .no_count();
    while !basic_bar.is_finished() {
        basic_bar.print();
        sleep(Duration::from_millis(5));
        basic_bar.inc_by(0.5);
    }
    basic_bar.finish();

    // Test Spinner
    let mut spinner = Spinner::new().with_message("Processing...");
    for _ in 0..=100 {
        spinner.tick();
        spinner.print();
        sleep(Duration::from_millis(100));
    }
    spinner.finish("Complete!");
}
