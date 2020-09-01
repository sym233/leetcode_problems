/** 
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        let r49_f0 = (rand7() - 1) * 7 + (rand7() - 1);
        // [0, 49)
        
        if r49_f0 < 40 {
            return r49_f0 % 10 + 1;
        }
        
        return Self::rand10();
    }
}
