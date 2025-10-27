use std::time::{Duration};
use std::thread::{sleep};
use simpleprogress::{BasicProgressBar, Spinner};

fn main() {
    println!("=== Test 1: Basic Progress Bar (no arrow, no count) ===");
    let mut basic_bar = BasicProgressBar::new(100.0)
        .no_arrow()
        .no_count();
    while !basic_bar.is_finished() {
        basic_bar.print();
        sleep(Duration::from_millis(5));
        basic_bar.inc_by(0.5);
    }
    basic_bar.finish();

    println!("\n=== Test 2: Full-featured bar with rate ===");
    let mut rate_bar = BasicProgressBar::new(50.0)
        .with_rate();
    for _ in 0..50 {
        rate_bar.inc();
        rate_bar.print();
        sleep(Duration::from_millis(50));
    }
    rate_bar.finish();

    println!("\n=== Test 3: Custom width and characters ===");
    let mut custom_bar = BasicProgressBar::new(30.0)
        .width(30)
        .chars('█', '░', '▶');
    for _ in 0..30 {
        custom_bar.inc();
        custom_bar.print();
        sleep(Duration::from_millis(30));
    }
    custom_bar.finish();

    println!("\n=== Test 4: Percentage only (no bar) ===");
    let mut percent_only = BasicProgressBar::new(100.0)
        .no_bar()
        .no_count();
    for i in 0..=100 {
        percent_only.set(i as f64);
        percent_only.print();
        sleep(Duration::from_millis(20));
    }
    percent_only.finish();

    println!("\n=== Test 5: Spinner ===");
    let mut spinner = Spinner::new().with_message("Processing...");
    for _ in 0..30 {
        spinner.tick();
        spinner.print();
        sleep(Duration::from_millis(100));
    }
    spinner.finish("Complete!");

    println!("\n=== Test 6: Spinner with changing message ===");
    let mut spinner2 = Spinner::new();
    let tasks = ["Loading", "Parsing", "Building", "Finishing"];
    for task in tasks {
        spinner2.set_message(task);
        for _ in 0..10 {
            spinner2.tick();
            spinner2.print();
            sleep(Duration::from_millis(80));
        }
    }
    spinner2.finish("All done!");

    println!("\n=== Test 7: Thin progress bar ===");
    let mut thin_bar = BasicProgressBar::new(100.0)
        .width(20);
    for _ in 0..100 {
        thin_bar.inc();
        thin_bar.print();
        sleep(Duration::from_millis(10));
    }
    thin_bar.finish();

    println!("\n=== Test 8: Using set() instead of inc() ===");
    let mut jump_bar = BasicProgressBar::new(100.0);
    for i in (0..=100).step_by(10) {
        jump_bar.set(i as f64);
        jump_bar.print();
        sleep(Duration::from_millis(200));
    }
    jump_bar.finish();
}
