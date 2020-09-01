impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        struct Pos {
            x: i32,
            y: i32,
        }
        impl Pos {
            fn new(x: i32, y: i32) -> Self {
                return Self {
                    x,
                    y,
                };
            }
            fn add(&self, another: &Self) -> Self {
                return Self {
                    x: self.x + another.x,
                    y: self.y + another.y,
                };
            }
        }
        
        fn get(grid: &Vec<Vec<i32>>, pos: &Pos) -> i32 {
            return grid[pos.x as usize][pos.y as usize];
        }
        
        let dir = [
            Pos::new(0, 1),
            Pos::new(0, -1),
            Pos::new(1, 0),
            Pos::new(-1, 0),
        ];
        
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        
        let is_in = |pos: &Pos| 0 <= pos.x && (pos.x as usize) < m && 0 <= pos.y && (pos.y as usize) < n;
        
        let mut minute = 0;
        let mut fresh_oranges = 0;
        let mut curr_rotten_oranges: Vec<Pos> = Vec::new();
        
        for i in 0..m {
            for j in 0..n {
                let pos = Pos::new(i as i32, j as i32);
                match get(&grid, &pos) {
                    1 => fresh_oranges += 1,
                    2 => curr_rotten_oranges.push(pos),
                    _ => (),
                }
            }
        }
        
        loop {
            let mut next_rotten_oranges: Vec<Pos> = Vec::new();
            for pos in curr_rotten_oranges {
                for d in dir.iter() {
                    let p = pos.add(d);
                    if is_in(&p) && get(&grid, &p) == 1 {
                        grid[p.x as usize][p.y as usize] = 2;
                        fresh_oranges -= 1;
                        next_rotten_oranges.push(p);
                    }
                }
            }
            if next_rotten_oranges.is_empty() {
                if fresh_oranges == 0 {
                    return minute;
                } else {
                    return -1;
                }
            }
            minute += 1;
            
            curr_rotten_oranges = next_rotten_oranges;
        }
    }
}
