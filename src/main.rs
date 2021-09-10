/**********************************************************
*  PROJECT_NAME: Mutasi;
*  VERSION: 0.2.0;
*  AUTHOR: Notidman;
***********************************************************/
use std::env;
use console::style;
include!("game.rs");
include!("table.rs");

fn main()
{
    for arg in env::args().skip(1) {
        let arg: &str = arg.as_str();
        match arg 
        {
            "--version" | "-v" => println!("{}{}", "Mutasi version", style(" 1.5").green().bold()),

            "--table" | "-t" => show_table("d"),

            "--tableb" | "-tb" => show_table("b"),

            "--tableo" | "-to" => show_table("o"),

            "--tablex" | "-tx" => show_table("x"),

            "--start" | "-s" => game("d"),

            "--startb" | "-sb" => game("b"),

            "--starto" | "-so" => game("o"),

            "--startx" | "-sx" => game("x"),

            "--help" | "-h"  =>  println!("\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n",
            style("--help -h").red(),    " -> Show help commands.",
            style("--version -v").red(), " -> Show the version of the program.",
            style("--table -t").red(),   " -> Shows the multiplication table in decimal.",
            style("--tableb -tb").red(),   " -> Shows the multiplication table in binary.",
            style("--tableo -to").red(),   " -> Shows the multiplication table in octal.",
            style("--tablex -tx").red(),   " -> Shows the multiplication table in decimal.",
            style("--start -s").red(),   " -> Game with the multiplication table in hexadecimal.",
            style("--startb -sb").red(), " -> Game with multiplication table in binary.",
            style("--starto -so").red(),   " -> Game with multiplication table in octal.",
            style("--startx -sx").red(),   " -> Game with multiplication table in hexadecimal."),

            // BETA Timer
            "--timer" => timer(),

            _ => print!("Mutasi: {} {}", style("error:").red().bold(), "Not correct args!")
        };
    }
}