use std::io; // For reading user input
use colored::Colorize; // For colored console output
use rand::random_range; // For generating random computer choice

fn main() {
    // Initialize score tracking
    let mut computer_score = 0;
    let mut user_score = 0;
    let mut tie_score = 0;
    println!("{}", "Welcome to Rock, Paper, Scissors Game".on_bright_cyan());
    // Game loop - continues until user quits
    loop {
        println!("Pick: 0 = Rock, 1 = Paper, 2 = Scissors , type 'Quit' to stop playing");
    
        // Get user input
        let mut user_pick_string = String::new();

        io::stdin().read_line(&mut user_pick_string).expect("Please Enter A Valid Number");
        // Check if user wants to quit
        if user_pick_string == "Quit" || user_pick_string == "quit" {
            break;
        }
        let user_pick_number: i32 = user_pick_string.trim().parse().expect("Please Enter A Number");
        let computer_number = random_range(0..=2); // Generate random computer choice (0-2)

        // Convert user and computer numbers to choice strings
        let user_choice = choice_picker(user_pick_number);
        let computer_choice = choice_picker(computer_number);
        // Validate user input
        if user_choice =="Invalid Choice"{
            println!("{}","Please select a choice from the choices".on_yellow());
            return;
        }
        // Check if user wins
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
        {            // Computer wins            computer_score += 1;
            println!("{}", "You Lose!".red());
        } else {
            // It's a tie
            tie_score += 1;
            println!("{}", "You Tied!".yellow());
        }
        println!("User Chose : {user_choice}");
        println!("Computer Chose : {computer_choice}");
             println!("Score is User: {user_score} - Computer: {computer_score} -Ties : {tie_score} ");
    }
}

// Convert numeric input to choice string
fn choice_picker(choice: i32) -> &'static str {
    let choices = ["Rock", "Paper", "Scissors"];
    match choice {
        0 => choices[0],
        1 => choices[1],
        2 => choices[2],
        _ => "Invalid Choice",
    }
}
