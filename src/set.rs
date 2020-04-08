use super::new_field;
use super::count;

pub fn set(field: &Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let mut new_field = new_field::new_field(field[0].len(),field.len());
    for y in 0..field.len(){
        for x in 0..field[0].len(){
            let n = count::count(&field,x as i32,y as i32);
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
