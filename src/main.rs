use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, BufReader, Write};
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Local};

struct TimeKeeper {
    total_worked_time: Duration,
    last_clock_in_time: Option<SystemTime>,
    is_clocked_in: bool,
}

impl TimeKeeper {
    fn new() -> Self {
        let mut keeper = Self {
            total_worked_time: Duration::ZERO,
            last_clock_in_time: None,
            is_clocked_in: false,
        };
        keeper.load_logs();
        keeper
    }

    fn clock_in(&mut self) {
        if self.is_clocked_in {
            println!("Already clocked in!");
            return;
        }
        let now = SystemTime::now();
        self.last_clock_in_time = Some(now);
        self.is_clocked_in = true;
        self.log_action("Clocked In", now);
        println!("Clocked in at {}", Self::format_time(now));
    }

    fn clock_out(&mut self) {
        if !self.is_clocked_in {
            println!("Not currently clocked in!");
            return;
        }
        let now = SystemTime::now();
        if let Some(clock_in_time) = self.last_clock_in_time {
            self.total_worked_time += now.duration_since(clock_in_time).unwrap();
        }
        self.last_clock_in_time = None;
        self.is_clocked_in = false;
        self.log_action("Clocked Out", now);
        println!("Clocked out at {}", Self::format_time(now));
    }

    fn show_logs(&self) {
        println!("Punch History:");
        let file = File::open("punch_log.txt").unwrap_or_else(|_| {
            println!("No log file found.");
            File::create("punch_log.txt").unwrap()
        });
        for line in BufReader::new(file).lines() {
            if let Ok(log) = line {
                println!("{}", log);
            }
        }
    }

    fn total_time(&self) {
        let total_seconds = self.total_worked_time.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        println!(
            "Total Worked Time: {:02}:{:02}:{:02}",
            hours, minutes, seconds
        );
    }

    fn log_action(&self, action: &str, time: SystemTime) {
        let datetime: DateTime<Local> = time.into();
        let log_message = format!("{}: {}", action, datetime.format("%Y-%m-%d %H:%M:%S"));
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("punch_log.txt")
            .unwrap();
        writeln!(file, "{}", log_message).unwrap();
    }

    fn load_logs(&mut self) {
        // Open the log file, or create it if it doesn’t exist.
        let file = File::open("punch_log.txt").unwrap_or_else(|_| {
            File::create("punch_log.txt").expect("Failed to create log file")
        });
    
        // Iterate over the lines in the log file.
        for line in BufReader::new(file).lines() {
            // If the line is valid, attempt to parse it.
            if let Ok(log) = line {
                let parts: Vec<&str> = log.split(": ").collect(); // Split into action and timestamp.
    
                // Ensure the log entry has exactly two parts.
                if parts.len() == 2 {
                    let action = parts[0]; // Action (Clocked In / Out).
                    if let Ok(parsed_time) = DateTime::parse_from_str(parts[1], "%Y-%m-%d %H:%M:%S") {
                        let time: SystemTime = parsed_time.with_timezone(&Local).into();
    
                        // Update the internal state based on the parsed action.
                        match action {
                            "Clocked In" => {
                                self.last_clock_in_time = Some(time);
                                self.is_clocked_in = true;
                            }
                            "Clocked Out" => {
                                if let Some(clock_in_time) = self.last_clock_in_time {
                                    self.total_worked_time += time.duration_since(clock_in_time)
                                        .unwrap_or(Duration::ZERO); // Avoid panic on duration calculation.
                                    self.last_clock_in_time = None;
                                }
                                self.is_clocked_in = false;
                            }
                            _ => {
                                println!("Unknown action in log: {}", action); // Handle unexpected actions gracefully.
                            }
                        }
                    } else {
                        println!("Failed to parse timestamp: {}", parts[1]); // Handle parse errors.
                    }
                } else {
                    println!("Malformed log entry: {}", log); // Handle malformed entries.
                }
            }
        }
    }

    fn format_time(time: SystemTime) -> String {
        let datetime: DateTime<Local> = time.into();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

fn main() {
    let mut keeper = TimeKeeper::new();
    loop {
        println!("\nCommands: [clock_in, clock_out, show_logs, total_time, exit]");
        print!("Enter command: ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "clock_in" => keeper.clock_in(),
            "clock_out" => keeper.clock_out(),
            "show_logs" => keeper.show_logs(),
            "total_time" => keeper.total_time(),
            "exit" => break,
            _ => println!("Unknown command!"),
        }
    }
}
