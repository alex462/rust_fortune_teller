use std::io;

fn main() {
    println!("Do you want to play a game? y/n: ");

    let mut user_plays = String::new();

    io::stdin().read_line(&mut user_plays)
        .expect("Failed to read line");

    let opt_in = String::from("y");
    let opt_out = String::from("n");

    takes_str(&opt_in);

    // if user_plays == opt_in {
    //     play_game();
    // }
    // if user_plays == opt_out {
    //     println!("Human forfeits. Computer wins by default.");
    // }
}

// fn play_game() {
fn takes_str(opt_in: &str) {
    let fortunes = ["run.", "No snowflake in an avalanche ever feels responsible.",
        "Your pet is planning to eat you.",
        "Your resemblance to a muppet will prevent the world from taking you seriously.",
        "If you eat something, and nobody sees you eat it, it has no calories.",
        "If you think no one cares if you are alive, try missing a couple car payments.",
        "A true patriot is the fellow who gets a parking ticket and rejoices that the system works. Are you a TRUE patriot?,",
        "Your friends secretly agree that your head is the wrong size for your body."];

    println!("{}", fortunes[7]); //should output 8th fortune in array of fortunes
}
// }
