use std::io;
use colored::Colorize;
use rand::random_range;


fn main() {
loop {
        println!("Pick: 0 = Rock, 1 = Paper, 2 = Scissors , type 'Quit' to stop playing");
        let computer_score =0;
        let user_score = 0;
    let mut user_pick_string = String::new();
    
    io::stdin().read_line(&mut user_pick_string).expect("Please Enter A Valid Number");
    if user_pick_string=="Quit"||user_pick_string=="quit"{
        println!("Thank you for playing! Final Score is User: {user_score} - Computer: {computer_score}");
        break
    }
    let user_pick_number :i32= user_pick_string.trim().parse().expect("Please Enter A Number");
    let computer_number = random_range(0..=2);


    let user_choice =choice_picker(user_pick_number);
    let computer_choice = choice_picker(computer_number);
    println!("User Chose : {user_choice}");
    println!("Computer Chose : {computer_choice}");
    if (computer_choice =="Rock" && user_choice=="Paper")||(computer_choice=="Paper" && user_choice=="Scissors")||(computer_choice=="Scissors" && user_choice=="Rock"){
        println!("{}","You Win".green() )

    }

}
    

}


fn choice_picker(choice:i32)-> &'static str{
    let choices= ["Rock", "Paper","Scissors"];
    match choice {
        0=>choices[0],
        1=>choices[1],
        2=>choices[2],
        _=>"Invalid Choice"

    }
}