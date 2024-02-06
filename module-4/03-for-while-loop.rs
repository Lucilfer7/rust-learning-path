fn code_loop() {
    let mut counter = 1;
    // stop_loop is set when loop stops

    let loop_stop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("Break the loop at counter = {}.", loop_stop);
}

fn code_while() {
    let mut counter = 1;
    while counter < 5 {
        println!("We loop a while...");
        counter = counter + 1;
    }
}

fn code_for() {
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
    for number in 0..5 {
        println!("{}", number * 2);
    }
}

fn main() {
    code_loop();
    code_while();
    code_for();
}
