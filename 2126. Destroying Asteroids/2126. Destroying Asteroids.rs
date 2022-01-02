impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut asteroids = asteroids;
        let mut mass = mass;
        asteroids.sort();
        for &a in &asteroids {
            if a <= mass {
                mass += a;
                if mass > *asteroids.last().unwrap() {
                    return true;
                }
            } else {
                return false;
            }
        }
        return true;
    }
}
