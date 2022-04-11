struct Solution {}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();
        let modk = (k as usize) % (n*m);
        let mut index = 0;
        let mut shifted_grid : Vec<Vec<i32>> = vec![vec![0; m]; n];

        for i in 0..n {
            for j in 0..m {
                shifted_grid[i][j] = grid[i][j]
            }
        }
        
        if modk == 0 {
            return shifted_grid
        }
        
        while index < n*m {
            let coord1 = (index/m, index%m);
            let shifted_index = (index+modk) % (n*m);
            let coord2 = (shifted_index/m, shifted_index%m);
            
            shifted_grid[coord2.0][coord2.1] = grid[coord1.0][coord1.1];

            index += 1;
        }

        shifted_grid
    }
}

fn main() {
    let mut test : Vec<Vec<i32>> = Vec::new();
    let k = 1;

    test.push(vec![1,]);
    test.push(vec![4,]);

    println!("Shifting grid {:?} to the left {}th time", test, k);
    println!("\t-> {:?}", Solution::shift_grid(test, k));

}
