use std::io;

fn main() {
    println!("Do you want to play a game? y/n: ");

    let mut user_plays = String::new();

    io::stdin().read_line(&mut user_plays)
        .expect("Failed to read line");

    play_game();

    //match user_plays.cmp(){

    //}
}

fn play_game() {
    let fortunes = ["run.", "No snowflake in an avalanche ever feels responsible.",
        "Your pet is planning to eat you.",
        "Your resemblance to a muppet will prevent the world from taking you seriously.",
        "If you eat something, and nobody sees you eat it, it has no calories.",
        "If you think no one cares if you are alive, try missing a couple car payments.",
        "A true patriot is the fellow who gets a parking ticket and rejoices that the system works. Are you a TRUE patriot?,",
        "Your friends secretly agree that your head is the wrong size for your body."];

    println!("{}", fortunes[7]); //should output 8th fortune in array of fortunes
}
