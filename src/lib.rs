pub mod aoc_2024_01 {

    use std::io::BufRead;

    fn _aoc_2024_01_01(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
        a.sort_unstable();
        b.sort_unstable();

        std::iter::zip(a, b).fold(0, |acc, e| acc + (e.0 - e.1).abs())
    }

    fn _aoc_2024_01_02(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut right = std::collections::HashMap::<i32, i32>::new();

        for i in b.iter() {
            right.entry(*i).and_modify(|e| *e += 1).or_insert(1);
        }

        a.iter()
            .fold(0, |acc, i| acc + *i * *right.entry(*i).or_insert(0))
    }

    pub fn aoc_2024_01_01_example() {
        let dist = _aoc_2024_01_01(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);

        println!("Distance: {}", dist);
    }

    pub fn aoc_2024_01_01() {
        // let file_path = "data/2024/01-example.txt";
        let file_path = "data/2024/01.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let reader = std::io::BufReader::new(file);

        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();

        for line in reader.lines() {
            let nums: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect();
            a.push(nums[0]);
            b.push(nums[1]);
        }

        println!("Distance: {}", _aoc_2024_01_01(a, b))
    }

    pub fn aoc_2024_01_02() {
        // let file_path = "data/2024/01-example.txt";
        let file_path = "data/2024/01.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let reader = std::io::BufReader::new(file);

        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();

        for line in reader.lines() {
            let nums: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect();
            a.push(nums[0]);
            b.push(nums[1]);
        }

        println!("Similarity: {}", _aoc_2024_01_02(a, b))
    }
}
