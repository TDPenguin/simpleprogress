use std::io::{Write};
use std::time::{Instant};

/// Configuration for progress bars.
/// Controls which elements are shown and the bar width.
pub struct BarConfig {
    /// Show the progress bar itself (the visual bar).
    pub show_bar: bool,
    /// Show the percentage completed (e.g. "42.0%").
    pub show_percentage: bool,
    /// Show the current/total count (e.g. "(42/100)").
    pub show_count: bool,
    /// Show the arrow at the front of the bar (e.g. "[===>    ]").
    pub show_arrow: bool,
    /// Show rate and ETA (e.g. "12/sec ETA: 5s").
    pub show_rate: bool,
    /// Width of the progress bar in characters.
    pub width: usize,
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            show_bar: true,
            show_percentage: true,
            show_count: true,
            show_arrow: true,
            show_rate: false,
            width: 50,
        }
    }
}

/// A basic, highly-configurable progress bar.
/// 
/// # Example
/// ```
/// use progress::BasicProgressBar;
/// 
/// let mut bar = BasicProgressBar::new(100.0);
/// bar.set(50.0);
/// bar.print();
/// ```
pub struct BasicProgressBar {
    current: f64,
    total: f64,
    config: BarConfig,
    start_time: Option<Instant>,
}

impl BasicProgressBar {
    /// Create a new progress bar with a given total.
    pub fn new(total: f64) -> Self {
        Self {
            current: 0.0,
            total,
            config: BarConfig::default(),
            start_time: None,
        }
    }

    /// Disables the arrow on the front of the loading bar.
    pub fn no_arrow(mut self) -> Self {
        self.config.show_arrow = false;
        self
    }

    /// Disables the percentage to the side of the loading bar.
    pub fn no_percentage(mut self) -> Self {
        self.config.show_percentage = false;
        self
    }

    /// Disables the counter to the side of the loading bar.
    pub fn no_count(mut self) -> Self {
        self.config.show_count = false;
        self
    }

    /// Disables the bar itself (only shows count/percentage).
    pub fn no_bar(mut self) -> Self {
        self.config.show_bar = false;
        self
    }

    /// Enables rate and ETA display on the progress bar.
    pub fn with_rate(mut self) -> Self {
        self.config.show_rate = true;
        self.start_time = Some(Instant::now());
        self
    }

    /// Increment the progress by 1.0.
    pub fn inc(&mut self) {
        self.current += 1.0;
    }

    /// Increment the progress by a specified amount.
    pub fn inc_by(&mut self, amount: f64) {
        self.current += amount;
    }

    /// Set the progress to a specific value.
    pub fn set(&mut self, value: f64) {
        self.current = value;
    }

    /// Render the progress bar as a string.
    /// Useful for custom printing or logging.
    pub fn render(&self) -> String {
        let mut parts = Vec::new();
        
        // Bar part
        if self.config.show_bar {
            let percentage = if self.total == 0.0 { 0.0 }
            else { self.current / self.total };

            let filled = (percentage * self.config.width as f64) as usize;
            let filled = filled.min(self.config.width);

            let bar: String = (0..self.config.width)
                .map(|i| {
                    if self.config.show_arrow {
                        if i < filled.saturating_sub(1) {
                            '='
                        } else if i == filled.saturating_sub(1) && filled > 0 {
                            '>'
                        } else {
                            ' '
                        }
                    } else {
                        // Without arrow: fill completely
                        if i < filled {
                            '='
                        } else {
                            ' '
                        }
                    }
                })
                .collect();

            parts.push(format!("[{}]", bar));
        }
        
        // Percentage part
        if self.config.show_percentage {
            let percentage = if self.total == 0.0 { 0.0 } 
                else { (self.current / self.total) * 100.0 };
            parts.push(format!("{:.2}%", percentage));
        }

        // Count part
        if self.config.show_count {
            parts.push(format!("({:.2}/{:.2})", self.current, self.total));
        }

        // Rate and ETA
        if self.config.show_rate {
            let elapsed = self.start_time.map(|t| t.elapsed().as_secs_f64()).unwrap_or(0.0);
            let rate = if elapsed > 0.0 { self.current / elapsed } else { 0.0 };
            parts.push(format!("{:.2}/sec", rate));
            if rate > 0.0 && self.current < self.total {
                let remaining = self.total - self.current;
                let eta_secs = remaining / rate;
                parts.push(format!("ETA: {}s", eta_secs as u64));
            }
        }

        parts.join(" ")
    }

    /// Print the progress bar to stdout, overwriting the current line.
    pub fn print(&self) {
        print!("\r{}\x1b[K", self.render());
        std::io::stdout().flush().unwrap();
    }

    /// Returns true if the progress bar has reached or exceeded the total.
    pub fn is_finished(&self) -> bool {
        self.current >= self.total
    }

    /// Print the final state of the progress bar and move to the next line.
    pub fn finish(&self) {
        println!("\r{}", self.render());
    }
}

/// A simple animated spinner for indicating ongoing work.
/// 
/// # Example
/// ```
/// use progress::Spinner;
/// 
/// let mut spinner = Spinner::new();
/// // update spinner.current_frame and print in a loop
/// ```
pub struct Spinner {
    frames: Vec<char>,
    current_frame: usize,
    message: String,
}

impl Spinner {
    pub fn new() -> Self {
        Self { 
            frames: vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            current_frame: 0,
            message: String::new(),
        }
    }

    pub fn with_message(mut self, msg: &str) -> Self {
        self.message = msg.to_string();
        self
    }

    pub fn tick(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.frames.len();
    }

    pub fn render(&self) -> String {
        format!("{} {}", self.frames[self.current_frame], self.message)
    }

    pub fn print(&self) {
        print!("\r{}\x1b[K", self.render());
        std::io::stdout().flush().unwrap();
    }

    pub fn finish(&self, final_msg: &str) {
        println!("\r{}", final_msg);
    }
}