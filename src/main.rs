extern crate rand;
extern crate termcolor;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn greeting() {
    println!(
        "\nRust TicTacToe\n\
         --------------\n\
         A simple game written in the rust programming language.\n\
         Code is available at: https://github.com/flofriday/tictactoe"
    )
}

fn print_player(player: &char) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    if player == &'X' {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
            .unwrap();
    } else if player == &'O' {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .unwrap();
    }

    write!(&mut stdout, "{}", player).unwrap();
    stdout.reset().unwrap();
}

fn draw(state: &[char]) {
    println!("\n");

    for i in (0..3).rev() {
        let offset = i * 3;

        print!("-------------\n| ");
        print_player(&state[offset]);
        print!(" | ");
        print_player(&state[offset + 1]);
        print!(" | ");
        print_player(&state[offset + 2]);
        println!(" |");
    }

    println!("-------------");
}

fn random_move(state: &[char]) -> usize {
    loop {
        let pos = rand::random::<usize>() % 10;
        if state[pos] != 'O' && state[pos] != 'X' {
            return pos;
        }
    }
}

fn block_move(state: &[char]) -> Option<usize> {
    for tmp in 0..3 {
        if state[tmp] == state[tmp + 3]
            || state[tmp] == state[tmp + 6]
            || state[tmp + 3] == state[tmp + 6]
        {
            for x in [0, 3, 6].iter() {
                if state[tmp + x] != 'O' && state[tmp + x] != 'X' {
                    return Some(tmp + x);
                }
            }
        }

        let tmp = tmp * 3;

        if state[tmp] == state[tmp + 1]
            || state[tmp] == state[tmp + 2]
            || state[tmp + 1] == state[tmp + 2]
        {
            for x in [0, 1, 2].iter() {
                if state[tmp + x] != 'O' && state[tmp + x] != 'X' {
                    return Some(tmp + x);
                }
            }
        }
    }

    if state[0] == state[4] || state[0] == state[8] || state[4] == state[8] {
        for idx in [0 as usize, 4, 8].iter() {
            if state[*idx] != 'O' && state[*idx] != 'X' {
                return Some(*idx);
            }
        }
    }

    if state[2] == state[4] || state[2] == state[6] || state[4] == state[6] {
        for idx in [2 as usize, 4, 6].iter() {
            if state[*idx] != 'O' && state[*idx] != 'X' {
                return Some(*idx);
            }
        }
    }
    None
}

fn random_corner(state: &[char], corners_idx: &[usize; 4]) -> Option<usize> {
    for corner in corners_idx {
        if state[*corner] != 'O' && state[*corner] != 'X' {
            return Some(*corner);
        }
    }
    None
}

fn ai_move(state: &[char], turn: usize) -> usize {
    let corners = ['1', '3', '7', '9'];
    let corners_idx = [0, 2, 6, 8];
    //We assign higher priority to corners
    match turn {
        1 => match random_corner(state, &corners_idx) {
            Some(x) => x,
            None => random_move(state),
        },
        2 => {
            for (idx, targets) in corners_idx.iter().enumerate() {
                //Check if player played a corner
                if state[*targets] != corners[idx] {
                    return 4;
                }
            }
            match random_corner(state, &corners_idx) {
                Some(x) => x,
                None => random_move(state),
            }
        }
        _ => match block_move(state) {
            None => match random_corner(state, &corners_idx) {
                Some(x) => x,
                None => random_move(state),
            },
            Some(x) => x,
        },
    }
}

fn ask_user(state: &mut [char], player: char) {
    loop {
        print!("Player '");
        print_player(&player);
        println!("', enter a number: ");

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Couldn't read line! Try again.");
            continue;
        }

        if let Ok(number) = input.trim().parse::<usize>() {
            if number < 1 || number > 9 {
                println!("The field number must be between 1 and 9.");
                continue;
            }

            let number = number - 1;

            if state[number] == 'X' || state[number] == 'O' {
                print!("This field is already taken by '");
                print_player(&state[number]);
                println!("'.");
                continue;
            }

            state[number] = player;

            break;
        } else {
            println!("Only numbers are allowed.");
            continue;
        }
    }
}

fn has_won(state: &[char]) -> bool {
    for tmp in 0..3 {
        if state[tmp] == state[tmp + 3] && state[tmp] == state[tmp + 6] {
            return true;
        }

        let tmp = tmp * 3;

        if state[tmp] == state[tmp + 1] && state[tmp] == state[tmp + 2] {
            return true;
        }
    }

    if (state[0] == state[4] && state[0] == state[8])
        || (state[2] == state[4] && state[2] == state[6])
    {
        return true;
    }

    false
}

#[inline(always)]
fn is_over(state: &[char]) -> bool {
    state.iter().all(|&v| v == 'X' || v == 'O')
}

fn main() {
    let mut state = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut player = 'X';

    // Welcome the player
    greeting();

    let ai_player;
    let mut user_move = match rand::random::<usize>() % 2 {
        0 => {
            println!(" AI goes second.");
            ai_player = 'O';
            true
        }
        _ => {
            println!(" AI goes first.");
            ai_player = 'X';
            false
        }
    };

    let mut turn: usize = 0;
    loop {
        // Draw the field
        draw(&state);

        turn += 1;
        if user_move {
            // Ask for user input
            ask_user(&mut state, player);
            user_move = false;
        } else {
            println!("AI made the Move 🤖");
            state[ai_move(&state, turn)] = ai_player;
            user_move = true;
        }

        // Check if a player won
        if has_won(&state) {
            draw(&state);
            print!("Player '");
            print_player(&player);
            println!("' won! \\(^.^)/");
            break;
        }

        // Check if all fields are used
        if is_over(&state) {
            draw(&state);
            println!("All fields are used. No one won. (._.)");
            break;
        }

        // Switch player
        player = if player == 'X' { 'O' } else { 'X' }
    }
}
