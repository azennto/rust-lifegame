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
