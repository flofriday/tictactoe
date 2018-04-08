use std::io;

fn draw(state: &[char]) {
    println!("\n\n");

    for i in 0..3 {
        let offset = i * 3;
        println!("-------------");
        println!("| {} | {} | {} |", state[0 + offset], state[1 + offset],
                 state[2 + offset]);
    }
    println!("-------------");
}

fn ask_user(state: &mut [char], player: char) {
        loop {
            println!("Player {} enter a number: ", player);

            let mut input = String::new();

            io::stdin().read_line(&mut input)
                .expect("Failed to read line");

            let number: usize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Only numbers are allowed!");
                    continue;
                }
            };

            if number < 1 && number > 9 {
                println!("The number musst be between 1 and 9");
                continue;
            }

            if state[number - 1] == 'O' || state[number - 1] == 'X' {
                println!("This Number was allready taken by: {}", state[number - 1]);
                continue;
            }

            state[number - 1] = player;
            break;
        }
}

fn has_won(state: &[char]) -> bool {
        for tmp in 0..3 {
            if state[0 + tmp] == state[3 + tmp] && 
                state[0 + tmp] == state[6 + tmp]{
                    return true;
                }
            if state[0 + tmp * 3] == state[1 + tmp * 3] && 
                state[0 + tmp * 3] == state[2 + tmp * 3]{
                    return true;
                }
        }

        if (state[0] == state[4] && state[0] == state[8]) ||
            (state[2] == state[4] && state[2] == state[6]) {
                return true;
            }

        false
}

fn is_over(state: &[char]) -> bool {
    for element in state.iter() {
        if element != &'X' && element != &'O' {
            return false;
        }
    }

    true
}

fn main() {
    let mut state = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut player : char = 'X';

    loop {

        // Draw the field
        draw(&state);

        // Ask for User input
        ask_user(&mut state, player);

        // Check if a player won
        if has_won(&state) == true {
            draw(&state);
            println!("Player {} won! \\(^.^)/", player);
            return;
        }

        // Check if all fields are used
        if is_over(&state) == true {
            draw(&state);
            println!("All fields are used. Noone won!");
            return;
        }

        // Set to next player
        if player == 'X' {
            player = 'O';
        } else {
            player = 'X';
        }

    }
}
