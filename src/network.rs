use rand::{self, Rng};
use rl_math_lib;

#[allow(unused)]
pub fn grid() -> Vec<Vec<char>> {
    vec![
        vec!['P', 'X', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'X', ' '],
        vec![' ', 'X', ' ', 'X', ' '],
        vec![' ', 'X', ' ', ' ', ' '],
        vec![' ', 'X', ' ', 'X', ' '],
        vec![' ', 'X', ' ', ' ', 'G'],
    ]
}


#[allow(unused)]
pub fn custom_max(arr: &Vec<Vec<f64>>, index: usize) -> f64 {
    let mut max = f64::MIN;
    let arr_slice = &arr[index];
    for i in arr_slice.iter() {
        if i > &max {
            max = *i;
        }
    }
    max
}

#[allow(unused)]
pub fn custom_argmax(arr: &Vec<Vec<f64>>, index: usize) -> usize {
    let mut max = f64::MIN;
    let mut max_index = usize::MIN;
    let arr_slice = &arr[index];
    for (i, item) in arr_slice.iter().enumerate() {
        if item > &max {
            max = *item;
            max_index = i; 
        }
    }
    max_index
}

#[allow(unused)]
fn get_possible_states(game: &Vec<Vec<char>>) -> usize {
    game.len() * game[0].len()
}

#[allow(unused)]
fn get_state(game: &Vec<Vec<char>>, player_pos: (usize, usize)) -> usize {
    let row_len = game[0].len();
    player_pos.0 * row_len + player_pos.1
}

#[allow(unused)]
fn create_q_table(states: &usize, actions: &usize) -> Vec<Vec<f64>> {
    let mut q_slice: Vec<f64> = vec![];
    let mut q_table: Vec<Vec<f64>> = vec![];
    for _ in 0..actions.clone() {
        q_slice.push(0.0);
    }
    for _ in 0..states.clone() {
        q_table.push(q_slice.clone())
    }
    q_table 
}


#[allow(unused)]
fn flat(game: &Vec<Vec<char>>) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for i in game.iter() {
        for j in i.iter() {
            result.push(*j);
        }
    }
    result
}

#[allow(unused)]
pub fn train(game: &Vec<Vec<char>>, iterations: usize, learning_rate: f64, discount_factor: f64, max_steps: usize) -> Result<Vec<Vec<f64>>, String> {

    let states = get_possible_states(&game);
    let actions = 4;
    let mut q_table = create_q_table(&states, &actions);
    let mut game_count = 0;
    for epoch in 0..=iterations {
        let player_pos = rl_math_lib::find(&game, 'P')?;
        let mut state = get_state(game, player_pos);

        for step in 0..=max_steps {
            let mut rng = rand::thread_rng();
            let rng_num = rng.gen_range(0.0..=1.0);

            let action = match rng_num < learning_rate {
                true => rng.gen_range(0..actions),
                false => custom_argmax(&q_table, state),
            };
            //println!("argmax: {}", rl_math_lib::argmax(&q_table, state));
            //println!("q_table manual: {:?}", &q_table[state]);
            //println!("argmax custom: {}", custom_argmax(&q_table, state));
            let (xrow, xcol) = rl_math_lib::divmod(state, game[0].len());
            let mut row = xrow as isize;
            let mut col = xcol as isize;
            //println!("row: {}", row);
            //println!("action: {}", action);
            match action {
                // up
                0 => row -= 1,
                // down
                1 => row += 1,
                // left
                2 => col -= 1,
                // right
                3 => col += 1,
                // error
                _ => return Err(String::from("Invalid Action!"))
            }
            let mut reward = 0.0;
            let new_state = if row < 0 || col < 0 {
                reward = -100.0;
                state
            } else if game.len() <= row as usize || game[0].len() <= col as usize {
                reward = -100.0;
                state
            } else {
                row as usize * game[0].len() + col as usize
            };

            if flat(&game)[new_state] == 'X' {
                reward += -100.0;
            } else if flat(&game)[new_state] == 'G' {
                println!("Finished Game: {}", game_count);
                game_count += 1;
                reward += 100.0;
                break;
            } else {
                reward += -1.0;
            }

            q_table[state][action] += learning_rate * (
                reward + discount_factor * custom_max(&q_table, new_state) - q_table[state][action]
            );

            //println!(" --- --- ");
            //println!("reward: {reward}");
            //println!("discount: {discount_factor}");
            //println!("learning_rate: {learning_rate}");
            //println!("max: {}", rl_math_lib::max(&q_table, new_state));
            //println!("max manual: {:#?}", &q_table[new_state]);
            //println!("custom max: {}", custom_max(&q_table, new_state));
            //println!("q_table old index: {}", &q_table[state][action]);
            state = new_state;
            if epoch == iterations {
                let mut game_field = game.clone();
                if row < 0 {
                    println!("row too low! {}", row);
                }
                if col < 0 {
                    println!("col too low! {}", col);
                } else {
                    if row >= 0 {

                    game_field[row as usize][col as usize] = 'B';
                    for i in game_field.iter() {
                        println!("{:?}", i);
                    }
                    println!("--- ---");
                    }
                }
                
            }
        }
    }

    Ok(q_table)
}