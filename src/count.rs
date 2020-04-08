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
