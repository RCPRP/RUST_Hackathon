//to create a productivity tool - Pomodoro app
/*The Pomodoro Technique is a popular time management method, and implementing it in Rust will be both fun and useful. Below is a step-by-step guide to creating a simple CLI Pomodoro timer application in Rust.
 Pomodoro Technique: 25 minutes of focused work followed by a 5-minute break, repeating this cycle until a total of 2 hours is reached.*/

use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn clear_screen() {
    // Clear the terminal screen by printing new lines
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}

fn countdown(minutes: u64, message: &str, cycle_number: Option<usize>) {
    let total_duration = Duration::from_secs(minutes * 60);
    let start_time = Instant::now();

    while start_time.elapsed() < total_duration {
        let elapsed = start_time.elapsed();
        let remaining = total_duration - elapsed;
        let minutes = remaining.as_secs() / 60;
        let seconds = remaining.as_secs() % 60;

        clear_screen();
        match cycle_number {
            Some(cycle) => println!("Cycle {}: {} - Time remaining: {:02}:{:02}", cycle, message, minutes, seconds),
            None => println!("{} - Time remaining: {:02}:{:02}", message, minutes, seconds),
        }
        io::stdout().flush().unwrap();

        sleep(Duration::from_secs(1));
    }
}

fn display_menu() {
    clear_screen();
    println!("Pomodoro Timer");
    println!("1. Start Pomodoro Cycles");
    println!("2. Exit");
    print!("Choose an option: ");
    io::stdout().flush().unwrap();
}

fn main() {
    let work_duration = 25; // in minutes
    let short_break_duration = 5; // in minutes
    let long_break_duration = 15; // in minutes
    let cycles_before_long_break = 4; // Number of work/break cycles before long break

    loop {
        display_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                let mut cycles = 0;

                loop {
                    cycles += 1;

                    // Work session
                    clear_screen();
                    println!("Cycle {}: Starting Work Timer...", cycles);
                    io::stdout().flush().unwrap();
                    countdown(work_duration, "Work", Some(cycles));
                    println!("Work session completed!");
                    io::stdout().flush().unwrap();

                    // Short break
                    clear_screen();
                    println!("Cycle {}: Starting Short Break Timer...", cycles);
                    io::stdout().flush().unwrap();
                    countdown(short_break_duration, "Short Break", Some(cycles));
                    println!("Short break completed!");
                    io::stdout().flush().unwrap();

                    // Check if it's time for a long break
                    if cycles % cycles_before_long_break == 0 {
                        clear_screen();
                        println!("Cycle {}: Starting Long Break Timer...", cycles);
                        io::stdout().flush().unwrap();
                        countdown(long_break_duration, "Long Break", Some(cycles));
                        println!("Long break completed!");
                        io::stdout().flush().unwrap();
                    }
                }
            }
            2 => break,
            _ => {
                clear_screen();
                println!("Invalid choice. Please try again.");
                io::stdout().flush().unwrap();
            }
        }
    }
}
