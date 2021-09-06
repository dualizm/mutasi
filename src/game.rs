
use rand::{thread_rng, Rng};
use std::io::Write;
use console::Term;
use std::thread;
use std::time::Duration;
use std::cmp::Ordering;

fn game() {

    let mut term = Term::stdout();
    let mut _correct = 0;
    let mut _wrong = 0;
    println!("{}", "Enter your answer(-1 to exit): ");

    loop {

        let x = thread_rng().gen_range(2..10);
        let y = thread_rng().gen_range(2..10);
        let answer = x * y;
        term.write( format!("Score: Correct: {} Incorrectly: {}\n", _correct, _wrong).as_bytes()).expect("noexpect");

        let expression = format!("Expression: {} * {} = ", x, y);

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

        thread::sleep(Duration::from_millis(1000));

        term.clear_last_lines(2)
        .expect("Not clear");
    }
}