use console::*;

fn line_char(system: &str) {
    println!("+{}+", "-".repeat(        
        match system { 
        "d" => 57,
        "b" => 97,
        "o" => 81,
        _ => panic!("{}", style("Error arg!").red()),}));
}

fn line_expression(system: &str, i: &i32, j: &i32, rl: i32) {
    match system {
        "d" => print!("| {} * {} = {:2} |{}", j, i, j * i, if j % rl == 0 {'\n'} else {' '} ),
        "b" => print!("| {:#6b} * {:#6b} = {:2} |{}", j, i, j * i, if j % rl == 0 {'\n'} else {' '} ),
        "o" => print!("| {:#4o} * {:#4o} = {:2} |{}", j, i, j * i, if j % rl == 0 {'\n'} else {' '} ),
        _ => panic!("{}", style("Error arg!").red()),
    }
}

pub fn show_table(system: &str) {
    let mut i = 2;
    line_char(system);
    while i < 10
    {
        let mut j = 2;
        while j <= 5
        {
            line_expression(system, &i, &j, 5);
            j += 1;
        }
        i += 1;
    }

    let mut i = 2;
    line_char(system);
    while i < 10
    {
        let mut j = 6;
        while j <= 9
        {
            line_expression(system, &i, &j, 9);
            j += 1;
        }
        i += 1;
    }
    line_char(system);
}
