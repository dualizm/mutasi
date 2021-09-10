
use rand::{thread_rng, Rng};
use std::io::Write;
use console::Term;
use std::thread;
use std::time::Duration;
use std::cmp::Ordering;

struct Timer { minutes: i32, seconds: i32}
impl Timer {
    fn new() -> Timer {
        Timer {
            minutes: 0,
            seconds: 0,
        }
    }

    fn set_time(&mut self, user_m: i32, user_s: i32) {
        self.minutes = user_m;
        self.seconds = user_s;
    }

    fn time_up(&mut self) {
        if self.seconds >= 60 {
            self.seconds = 0;
            self.minutes += 1;
        }
        else {
            self.seconds += 1;
        }
    }

    fn time_dw(&mut self) {
        if self.seconds <= 0 {
            self.minutes -= 1;
            self.seconds = 60;
        }
        else {
            self.seconds -= 1;
        }
    }

    fn get_data(&self) -> String {
        format!("Time: {:02} : {:02}", self.minutes, self.seconds)
    }
}

fn timer() {
    let mut term = Term::stdout();
    let mut timer = Timer::new();
    loop {
        term.write(timer.get_data().as_bytes()).expect("Some error");
        thread::sleep(Duration::from_millis(1000));
        term.clear_line()
        .expect("Not clear");
        timer.time_up();
    }
}

fn game(gamemode: &str) {

    let mut term = Term::stdout();
    let mut _correct = 0;
    let mut _wrong = 0;
    println!("{}", "Enter your answer(-1 to exit): ");

    loop {

        let x = thread_rng().gen_range(2..10);
        let y = thread_rng().gen_range(2..10);
        let answer = x * y;
        term.write( format!("Score: Correct: {} Incorrectly: {}\n", _correct, _wrong).as_bytes()).expect("noexpect");
        
        let expression: String;
        match gamemode {
            "d" => expression = format!("Expression: {} * {} = ", x, y),
            "b" => expression = format!("Expression: {:#b} * {:#b} = ", x, y),
            "o" => expression = format!("Expression: {:#o} * {:#o} = ", x, y),
            "x" => expression = format!("Expression: {:#x} * {:#x} = ", x, y),
            _ => panic!("{}", style("Error arg!").red()),
        }

        term.write(expression.as_bytes()).expect("impossible to withdraw expression");

        let mut guess: String = String::new();
        std::io::stdin().read_line(&mut guess).expect("");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                term.write( format!("{}", style("Incorrectly!").red().bold()).as_bytes() )
                .expect("wut");
                _wrong += 1;
                term.clear_last_lines(1)
                .expect("Not clear");
                continue;
            },
        };

        if guess == -1 {
            println!("See you soon!");
            std::process::exit(0);
        }

        match guess.cmp(&answer) {
            Ordering::Equal => {
                term.write( format!("{}", style("Correct!    ").green().bold()).as_bytes() )
                .expect("wut");
                _correct += 1;
            },
            _ => {
                term.write( format!("{}", style("Incorrectly!").red().bold()).as_bytes() )
                .expect("wut");
                _wrong += 1;
            },
        }

        term.clear_last_lines(2)
        .expect("Not clear");
    }
}