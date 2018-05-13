fn greeting() {
    println!(
        "\nRust TicTacToe\n\
         --------------\n\
         A simple game written in the rust programming language.\n\
         Code is available at: https://github.com/flofriday/tictactoe"
    )
}

#[cfg(target_os = "unix")]
fn fmt_player(player: &char) -> String {
    if player == &'X' {
        return "\x1b[34mX\x1b[0m".to_string();
    } else if player == &'O' {
        return "\x1b[32mO\x1b[0m".to_string();
    } else {
        return player.to_string();
    }
}

#[cfg(not(target_os = "unix"))]
fn fmt_player(player: &char) -> String {
    return player.to_string();
}

fn draw(state: &[char]) {
    println!("\n");

    for i in (0..3).rev() {
        let offset = i * 3;

        println!(
            "-------------\n\
             | {} | {} | {} |",
            fmt_player(&state[offset]),
            fmt_player(&state[offset + 1]),
            fmt_player(&state[offset + 2])
        );
    }

    println!("-------------");
}

fn ask_user(state: &mut [char], player: char) {
    loop {
        println!("Player '{}', enter a number: ", fmt_player(&player));

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
                println!(
                    "This field is already taken by '{}'.",
                    fmt_player(&state[number])
                );
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

    loop {
        // Draw the field
        draw(&state);

        // Ask for user input
        ask_user(&mut state, player);

        // Check if a player won
        if has_won(&state) {
            draw(&state);
            println!("Player '{}' won! \\(^.^)/", fmt_player(&player));
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
