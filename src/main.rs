use std::io;
use colored::Colorize;
use rand::random_range;

fn main() {
        let mut computer_score = 0;
        let mut user_score = 0;
        let mut tie_score = 0;
    println!("{}", "Welcome to Rock, Paper, Scissors Game".on_bright_cyan());
    loop {
        println!("Pick: 0 = Rock, 1 = Paper, 2 = Scissors , type 'Quit' to stop playing");
    
        let mut user_pick_string = String::new();

        io::stdin().read_line(&mut user_pick_string).expect("Please Enter A Valid Number");
        if user_pick_string == "Quit" || user_pick_string == "quit" {
            break;
        }
        let user_pick_number: i32 = user_pick_string.trim().parse().expect("Please Enter A Number");
        let computer_number = random_range(0..=2);

        let user_choice = choice_picker(user_pick_number);
        let computer_choice = choice_picker(computer_number);
        if user_choice =="Invalid Choice"{
            println!("{}","Please select a choice from the choices".on_yellow());
            return;
        }
        if
            (computer_choice == "Rock" && user_choice == "Paper") ||
            (computer_choice == "Paper" && user_choice == "Scissors") ||
            (computer_choice == "Scissors" && user_choice == "Rock")
        {
            user_score += 1;
            println!("{}", "You Win".green());
        } else if
            (computer_choice == "Rock" &&
                user_choice == "Scissors")||
               ( computer_choice == "Paper" &&
                user_choice == "Rock") ||
            (computer_choice == "Scissors" && user_choice == "Paper")
        {
            computer_score += 1;
            println!("{}", "You Lose!".red());
        } else {
            tie_score += 1;
            println!("{}", "You Tied!".yellow());
        }
        println!("User Chose : {user_choice}");
        println!("Computer Chose : {computer_choice}");
             println!("Score is User: {user_score} - Computer: {computer_score} -Ties : {tie_score} ");
    }
}

fn choice_picker(choice: i32) -> &'static str {
    let choices = ["Rock", "Paper", "Scissors"];
    match choice {
        0 => choices[0],
        1 => choices[1],
        2 => choices[2],
        _ => "Invalid Choice",
    }
}
