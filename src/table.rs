
fn show_table() {
    let mut i = 2;
    println!("+{}+", "-".repeat(57));
    while i < 10
    {
        let mut j = 2;
        while j <= 5
        {
            print!("| {} * {} = {:2} |{}", j, i, j * i, if j % 5 == 0 {'\n'} else {' '} );
            j += 1;
        }
        i += 1;
    }

    let mut i = 2;
    println!("+{}+", "-".repeat(57));
    while i < 10
    {
        let mut j = 6;
        while j <= 9
        {
            print!("| {} * {} = {:2} |{}", j, i, j * i, if j % 9 == 0 {'\n'} else {' '} );
            j += 1;
        }
        i += 1;
    }
    println!("+{}+", "-".repeat(57));
}