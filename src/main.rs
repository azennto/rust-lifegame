use std::time::Duration;

static DIST: [[i32; 2]; 8] = [
    [-1, 0], [-1, -1], [0, -1], [1, -1],
    [1, 0], [1, 1], [0, 1], [-1, 1]
];

pub fn count(field: &Vec<Vec<bool>> , x : i32 , y : i32) -> i32{
    let mut ans = 0;
    for n in 0..DIST.len(){
        let new_x = x + DIST[n][0];
        let new_y = y + DIST[n][1];
        if(0 <= new_x && new_x < field[0].len() as i32 && 0 <= new_y && new_y < field.len() as i32){
            ans += if field[new_x as usize][new_y as usize] == true {1}else{0};
        }
    }
    return ans;
}

pub fn new_field(width: usize,height: usize) -> Vec<Vec<bool>> {
    return vec![vec![false;width];height];
}

pub fn set(field: &Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let mut new_field = new_field(field[0].len(),field.len());
    for y in 0..field.len(){
        for x in 0..field[0].len(){
            let n = count(&field,x as i32,y as i32);
            if field[x][y] == true {
                new_field[x][y] = if n == 2 || n == 3 {
                    true //生存
                }else if n <= 1 {
                    false//過疎
                }else{
                    false//過密
                };
            }else{
                new_field[x][y] = if n == 3 {
                    true
                }else{
                    false
                };
            }
        }
    }
    return new_field;
}

pub fn print_field(field: &Vec<Vec<bool>>){
    print!("\x1B[2J");
    for y in 0..field.len(){
        for x in 0..field[0].len(){
            if field[x][y] {
                print!("#");
            }else{
                print!(".");
            }
        }
        println!("");
    }
}


fn main() {
    let mut game_field = new_field(40,40);
    
    game_field[20][20] = true;
    game_field[20][21] = true;
    game_field[20][22] = true;

    loop {
        print_field(&game_field);
        game_field = set(&game_field);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}
