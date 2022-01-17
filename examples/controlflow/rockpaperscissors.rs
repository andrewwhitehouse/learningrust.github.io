use crate::Winner::{PLAYER1, PLAYER2};

#[derive(PartialEq)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS
}

#[derive(PartialEq)]
enum Winner {
    PLAYER1,
    PLAYER2
}

fn rock_paper_scissors(player1_choice: Choice, player2_choice: Choice) -> Option<Winner> {
    if player1_choice == Choice::ROCK {
        if player2_choice == Choice::ROCK {
            None
        } else if player2_choice == Choice::SCISSORS { // Rock blunts scissors
            Some(PLAYER1)
        } else { // Paper wraps rock
            Some(PLAYER2)
        }
    } else if player1_choice == Choice::PAPER {
        if player2_choice == Choice::ROCK { // Paper wraps rock
            Some(PLAYER1)
        } else if player2_choice == Choice::SCISSORS { // Scissors cut paper
            Some(PLAYER2)
        } else { // Paper wraps rock
            None
        }
    } else {
        None
    }
}

fn main() {
    let result = rock_paper_scissors(Choice::ROCK,
                                     Choice::SCISSORS);
    match result {
      Some(PLAYER1) => println!("Player 1 wins"),
      Some(PLAYER2) => println!("Player 2 wins"),
      _ => println!("It's a draw")
    }
}
