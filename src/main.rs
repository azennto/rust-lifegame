use std::time::Duration;

mod count;
mod new_field;
mod print;
mod set;

fn main() {
    let mut game_field = new_field::new_field(40,40);
    
    game_field[20][20] = true;
    game_field[21][20] = true;
    game_field[21][21] = true;

    game_field[25][21] = true;
    game_field[26][21] = true;
    game_field[27][21] = true;

    game_field[26][19] = true;

    loop {
        print::print_field(&game_field);
        game_field = set::set(&game_field);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 3));
    }
}
