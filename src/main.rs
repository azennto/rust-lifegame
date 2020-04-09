use std::time::Duration;

mod count;
mod new_field;
mod print;
mod set;

fn main() {
    let mut game_field = new_field::new_field(60,60);
    
    game_field[20][20] = true;
    game_field[20][21] = true;
    game_field[21][20] = true;

    game_field[23][20] = true;
    game_field[24][20] = true;
    game_field[24][19] = true;
    game_field[24][18] = true;
    game_field[25][19] = true;

    loop {
        print::print_field(&game_field);
        game_field = set::set(&game_field);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}
