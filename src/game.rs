
use rand::{thread_rng, Rng};
use std::io::Write;
use console::*;
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

pub fn timer() {
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

pub struct Game{gamemode: String, correct: i32, wrong: i32}
impl Game {
    pub fn new(gamemode: &str) -> Game {
        Game {
            gamemode: gamemode.to_string(),
            correct: 0,
            wrong : 0,
        }
    }

    pub fn game(&mut self) {

        let mut term = Term::stdout();
        println!("{}", "Enter your answer(-1 to exit): ");

        loop {
            let (x, y) = Game::gen_num(&self.gamemode);
            
            let expression = Game::gen_expression(&self.gamemode, x, y);

            let answer = x * y;
            term.write( format!("Score: Correct: {} Incorrectly: {}\n", self.correct, self.wrong).as_bytes()).expect("noexpect");
    
            term.write(expression.as_bytes()).expect("impossible to withdraw expression");
    
            let mut guess: String = String::new();
            std::io::stdin().read_line(&mut guess).expect("");
    
            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    term.write( format!("{}", style("Incorrectly!").red().bold()).as_bytes() )
                    .expect("wut");
                    self.wrong += 1;
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
                    self.correct += 1;
                },
                _ => {
                    term.write( format!("{}", style("Incorrectly!").red().bold()).as_bytes() )
                    .expect("wut");
                    self.wrong += 1;
                },
            }
    
            term.clear_last_lines(2)
            .expect("Not clear");
        }

    }

    fn gen_num(gamemode: &String) -> (i32, i32) {

        let (x, y) = (
        if gamemode == "r" {
            thread_rng().gen_range(1..=100)
        }
        else {
            thread_rng().gen_range(1..10)
        },
        if gamemode == "r" {
            thread_rng().gen_range(1..=100)
        }
        else {
            thread_rng().gen_range(1..10)
        },);
        (x,y)
    } 

    fn gen_expression(gamemode: &String, x: i32, y: i32) -> String {
        match gamemode.as_str() {
            "d" => format!("Expression: {} * {} = ", x, y),
            "b" => format!("Expression: {:#b} * {:#b} = ", x, y),
            "o" => format!("Expression: {:#o} * {:#o} = ", x, y),
            "r" => format!("Expression: {:#b} * {:#b} = ", x, y),
            _ => panic!("{}", style("Error arg!").red())
        }
    }

}