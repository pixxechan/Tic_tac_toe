use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn to_char(self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
    
    fn other(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

struct Game {
    board: [Option<Player>; 9],
    current_player: Player,
}

impl Game {
    fn new() -> Self {
        Game {
            board: [None; 9],
            current_player: Player::X,
        }
    }
    
    fn display_board(&self) {
        println!("\n   |   |   ");
        for row in 0..3 {
            print!(" ");
            for col in 0..3 {
                let index = row * 3 + col;
                match self.board[index] {
                    Some(player) => print!("{}", player.to_char()),
                    None => print!("{}", index + 1),
                }
                if col < 2 {
                    print!(" | ");
                }
            }
            println!();
            if row < 2 {
                println!("___|___|___");
                println!("   |   |   ");
            }
        }
        println!("   |   |   \n");
    }
    
    fn make_move(&mut self, position: usize) -> bool {
        if position == 0 || position > 9 || self.board[position - 1].is_some() {
            return false;
        }
        
        self.board[position - 1] = Some(self.current_player);
        self.current_player = self.current_player.other();
        true
    }
    
    fn check_winner(&self) -> Option<Player> {
        // Check rows
        for row in 0..3 {
            let start = row * 3;
            if self.board[start].is_some() 
                && self.board[start] == self.board[start + 1] 
                && self.board[start + 1] == self.board[start + 2] {
                return self.board[start];
            }
        }
        
        // Check columns
        for col in 0..3 {
            if self.board[col].is_some() 
                && self.board[col] == self.board[col + 3] 
                && self.board[col + 3] == self.board[col + 6] {
                return self.board[col];
            }
        }
        
        // Check diagonals
        if self.board[0].is_some() && self.board[0] == self.board[4] && self.board[4] == self.board[8] {
            return self.board[0];
        }
        if self.board[2].is_some() && self.board[2] == self.board[4] && self.board[4] == self.board[6] {
            return self.board[2];
        }
        
        None
    }
    
    fn is_board_full(&self) -> bool {
        self.board.iter().all(|&cell| cell.is_some())
    }
    
    fn get_user_input(&self) -> Result<usize, std::num::ParseIntError> {
        println!("Player {} turn!", self.current_player.to_char());
        println!("Enter position (1-9): ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        input.trim().parse()
    }
}

fn main() {
    println!("=== Tic Tac Toe ===");
    println!("Enter numbers 1-9 to place your mark:");
    
    let mut game = Game::new();
    
    loop {
        game.display_board();
        
        // Check for winner
        if let Some(winner) = game.check_winner() {
            println!("🎉 Player {} wins!", winner.to_char());
            break;
        }
        
        // Check for tie
        if game.is_board_full() {
            println!("🤝 It's a tie!");
            break;
        }
        
        // Get user input
        match game.get_user_input() {
            Ok(position) => {
                if !game.make_move(position) {
                    println!("❌ Invalid move! Try again.");
                }
            }
            Err(_) => {
                println!("❌ Please enter a valid number between 1 and 9!");
            }
        }
    }
    
    game.display_board();
    println!("Thanks for playing!");
}
