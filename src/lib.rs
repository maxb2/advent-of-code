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

pub mod aoc_2024_02 {
    use std::io::BufRead;

    pub enum Direction {
        Ascending,
        Descending,
    }

    pub fn _aoc_2024_02_01<I>(reports: I) -> u32
    where
        I: IntoIterator<Item = Vec<i32>>,
    {
        let mut num_safe = 0;
        'report: for report in reports.into_iter() {
            let mut dir: Option<Direction> = None;
            for i in 1..report.len() {
                let diff = report[i - 1] - report[i];
                match dir {
                    Some(Direction::Ascending) => {
                        if diff <= 0 {
                            continue 'report;
                        }
                    }
                    Some(Direction::Descending) => {
                        if diff >= 0 {
                            continue 'report;
                        }
                    }
                    None => {
                        if diff > 0 {
                            dir = Some(Direction::Ascending)
                        } else {
                            dir = Some(Direction::Descending)
                        }
                    }
                }
                if diff.abs() < 1 || diff.abs() > 3 {
                    continue 'report;
                }
            }
            num_safe += 1;
        }
        return num_safe;
    }

    pub fn aoc_2024_02_01() {
        // let file_path = "data/2024/02-example.txt";
        let file_path = "data/2024/02.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let reader = std::io::BufReader::new(file);

        let reports = reader.lines().map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        });

        println!("Safe: {}", _aoc_2024_02_01(reports))
    }

    pub fn is_safe(diff: &Vec<i32>) -> bool {
        let dir = diff[0];
        let mut i = 0;

        while i < diff.len() {
            let d = diff[i];
            if dir * d < 0 || d.abs() < 1 || d.abs() > 3 {
                return false;
            }
            i += 1;
        }
        return true;
    }

    pub fn aoc_2024_02_02() {
        // let file_path = "data/2024/02-example.txt";
        let file_path = "data/2024/02.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let reader = std::io::BufReader::new(file);

        let reports = reader.lines().map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        });

        let num_safe = reports
            .filter(|report| {
                let mut safe = false;
                for i in 0..report.len() {
                    let _report = [&report[0..i], &report[i + 1..report.len()]]
                        .concat()
                        .to_vec();

                    let mut diff: Vec<i32> = Vec::new();
                    for i in 1.._report.len() {
                        diff.push(_report[i] - _report[i - 1])
                    }

                    safe |= is_safe(&diff);
                    if safe {
                        break;
                    }
                }
                safe
            })
            .count();

        println!("Safe: {}", num_safe);
    }
}

pub mod aoc_2024_03 {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::anychar;
    use nom::combinator::value;
    use nom::multi::{many0, many_till};
    use nom::sequence::{delimited, preceded, separated_pair};
    use nom::IResult;

    use std::io::Read;

    fn corrupted_parser(s: &str) -> IResult<&str, Vec<(Vec<char>, (i32, i32))>> {
        many0(many_till(
            anychar,
            preceded(
                tag("mul"),
                delimited(
                    tag("("),
                    separated_pair(
                        nom::character::complete::i32,
                        tag(","),
                        nom::character::complete::i32,
                    ),
                    tag(")"),
                ),
            ),
        ))(s)
    }

    pub fn aoc_2024_03_01() {
        // let file_path = "data/2024/03-example.txt";
        let file_path = "data/2024/03.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let mut reader = std::io::BufReader::new(file);

        let mut data: String = String::new();

        reader.read_to_string(&mut data).unwrap();

        let parsed = corrupted_parser(&data).unwrap();

        let res = parsed
            .1
            .iter()
            .map(|t| t.1)
            .fold(0, |acc, e| acc + e.0 * e.1);

        println!("Value: {}", res);
    }

    #[derive(Debug, Clone, Copy)]
    pub enum MulState {
        Do,
        Dont,
        Mul(i32, i32),
    }

    pub fn parse_mul(s: &str) -> IResult<&str, MulState> {
        let (remaining, (x, y)) = preceded(
            tag("mul"),
            delimited(
                tag("("),
                separated_pair(
                    nom::character::complete::i32,
                    tag(","),
                    nom::character::complete::i32,
                ),
                tag(")"),
            ),
        )(s)?;

        Ok((remaining, MulState::Mul(x, y)))
    }

    fn corrupted_enabled_parser(s: &str) -> IResult<&str, Vec<(Vec<char>, MulState)>> {
        let res = many0(many_till(
            anychar,
            alt((
                value(MulState::Do, tag("do()")),
                value(MulState::Dont, tag("don't()")),
                parse_mul,
            )),
        ))(s);

        res
    }

    pub fn aoc_2024_03_02() {
        // let file_path = "data/2024/03-example2.txt";
        let file_path = "data/2024/03.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let mut reader = std::io::BufReader::new(file);

        let mut data: String = String::new();

        reader.read_to_string(&mut data).unwrap();

        let parsed = corrupted_enabled_parser(&data).unwrap();

        let mut res = 0;

        let mut do_mul = true;

        for (_, mul) in parsed.1.iter() {
            match mul {
                MulState::Do => do_mul = true,
                MulState::Dont => do_mul = false,
                MulState::Mul(x, y) => {
                    if do_mul {
                        res += x * y;
                    }
                }
            }
        }

        println!("Value: {}", res);
    }
}

pub mod aoc_2024_04 {
    use std::io::BufRead;

    pub fn aoc_2024_04_01() {
        // let file_path = "data/2024/04-example.txt";
        let file_path = "data/2024/04.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let reader = std::io::BufReader::new(file);

        let mut data: Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            data.push(line.unwrap().chars().collect());
        }

        let key: Vec<char> = "XMAS".chars().collect();

        let key_len = key.len();

        let mut amount = 0u32;

        let height = data.len();
        let width = data[0].len();

        for (y, line) in data.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c != key[0] {
                    continue;
                }

                // let can_E = width - (x) >= key_len;
                // let can_W = x >= key_len;
                // let can_S = height - (y) >= key_len;
                // let can_N = (y) >= key_len;

                // check E
                let mut found = true;
                for z in 1..key_len {
                    if x + z >= width || data[y][x + z] != key[z] {
                        found = false;
                        break;
                    }
                }
                if found {
                    amount += 1;
                }

                // check NE
                found = true;
                for z in 1..key_len {
                    if (y as i32 - z as i32) < 0 || x + z >= width || data[y - z][x + z] != key[z] {
                        found = false;
                        break;
                    }
                }
                if found {
                    amount += 1;
                }

                // check N
                found = true;
                for z in 1..key_len {
                    if (y as i32 - z as i32) < 0 || data[y - z][x] != key[z] {
                        found = false;
                        break;
                    }
                }
                if found {
                    amount += 1;
                }

                // check NW
                found = true;
                for z in 1..key_len {
                    if (y as i32 - z as i32) < 0
                        || (x as i32 - z as i32) < 0
                        || data[y - z][x - z] != key[z]
                    {
                        found = false;
                        break;
                    }
                }
                if found {
                    amount += 1;
                }

                // check W
                found = true;
                for z in 1..key_len {
                    if (x as i32 - z as i32) < 0 || data[y][x - z] != key[z] {
                        found = false;
                        break;
                    }
                }
                if found {
                    amount += 1;
                }

                // check SW
                found = true;
                for z in 1..key_len {
                    if (x as i32 - z as i32) < 0 || y + z >= height || data[y + z][x - z] != key[z]
                    {
                        found = false;
                        break;
                    }
                }
                if found {
                    amount += 1;
                }

                // check S

                found = true;
                for z in 1..key_len {
                    if y + z >= height || data[y + z][x] != key[z] {
                        found = false;
                        break;
                    }
                }
                if found {
                    amount += 1;
                }

                // check SE
                found = true;
                for z in 1..key_len {
                    if y + z >= height || x + z >= width || data[y + z][x + z] != key[z] {
                        found = false;
                        break;
                    }
                }
                if found {
                    amount += 1;
                }
            }
        }

        println!("Amount: {}", amount)
    }

    pub fn aoc_2024_04_02() {
        // let file_path = "data/2024/04-example2.txt";
        let file_path = "data/2024/04.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let reader = std::io::BufReader::new(file);

        let mut data: Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            data.push(line.unwrap().chars().collect());
        }

        // let key: Vec<char> = "XMAS".chars().collect();

        // let key_len = key.len();

        let mut amount = 0u32;

        let height = data.len();
        let width = data[0].len();

        for (y, line) in data.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c != 'A'
                    || x + 1 >= width
                    || x as i32 - 1 < 0
                    || y + 1 >= height
                    || y as i32 - 1 < 0
                {
                    continue;
                }

                let northeast = data[y - 1][x + 1];
                let southeast = data[y + 1][x + 1];
                let northwest = data[y - 1][x - 1];
                let southwest = data[y + 1][x - 1];

                if ((northeast == 'M' && southwest == 'S')
                    || (northeast == 'S' && southwest == 'M'))
                    && ((northwest == 'M' && southeast == 'S')
                        || (northwest == 'S' && southeast == 'M'))
                {
                    amount += 1;
                }
            }
        }

        println!("Amount: {}", amount)
    }
}

pub mod aoc_2024_05 {
    use std::{collections::HashMap, io::BufRead};

    pub fn aoc_2024_05_01() {
        // let file_path = "data/2024/05-example.txt";
        let file_path = "data/2024/05.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let reader = std::io::BufReader::new(file);

        let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

        let empty: Vec<u32> = Vec::new();

        let mut acc = 0u32;

        'line: for line in reader.lines().map(|r| r.unwrap()) {
            if line.contains('|') {
                let nums: Vec<u32> = line.split('|').map(|e| e.parse::<u32>().unwrap()).collect();
                rules.entry(nums[1]).or_insert(vec![nums[0]]).push(nums[0]);
            }

            if line.contains(",") {
                let mut pages: Vec<u32> = Vec::new();
                for page in line.split(',').map(|e| e.parse::<u32>().unwrap()) {
                    // println!("Check: {}", page);
                    for previous_page in pages.iter() {
                        let prev_rule = rules.get(previous_page).unwrap_or(&empty);
                        // println!("{}?{} {}|{:?}",previous_page, page, previous_page, prev_rule);
                        if prev_rule.contains(&page) {
                            // println!("BAD");
                            continue 'line;
                        }
                    }

                    pages.push(page);
                }
                println!("GOOD: {}", line);
                let middle = pages[pages.len() / 2];
                println!("{}", middle);
                acc += middle;
            }
        }
        println!("Value: {}", acc);
    }

    pub struct Page {
        value: u32,
        before: Vec<u32>,
    }

    impl PartialOrd for Page {
        fn ge(&self, other: &Self) -> bool {
            !other.before.contains(&self.value)
        }
        fn lt(&self, other: &Self) -> bool {
            other.before.contains(&self.value)
        }

        fn gt(&self, other: &Self) -> bool {
            self.before.contains(&other.value)
        }

        fn le(&self, other: &Self) -> bool {
            !self.before.contains(&other.value)
        }

        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if other.before.contains(&self.value) {
                return Some(std::cmp::Ordering::Less);
            };

            if self.before.contains(&other.value) {
                return Some(std::cmp::Ordering::Greater);
            }

            Some(std::cmp::Ordering::Equal)
        }
    }

    impl PartialEq for Page {
        fn eq(&self, other: &Self) -> bool {
            !self.lt(other) && !other.lt(self)
        }
        fn ne(&self, other: &Self) -> bool {
            self.lt(other) || other.lt(self)
        }
    }

    pub fn aoc_2024_05_02() {
        // let file_path = "data/2024/05-example.txt";
        let file_path = "data/2024/05.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let reader = std::io::BufReader::new(file);

        let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

        let empty: Vec<u32> = Vec::new();

        let mut acc = 0u32;

        for line in reader.lines().map(|r| r.unwrap()) {
            if line.contains('|') {
                let nums: Vec<u32> = line.split('|').map(|e| e.parse::<u32>().unwrap()).collect();
                rules.entry(nums[1]).or_insert(vec![nums[0]]).push(nums[0]);
            }

            if line.contains(",") {
                let mut pages: Vec<u32> = Vec::new();
                let mut _pages: Vec<u32> =
                    line.split(',').map(|e| e.parse::<u32>().unwrap()).collect();
                let mut bad = false;
                'page: for page in _pages.iter() {
                    bad = false;
                    // println!("Check: {}", page);
                    for previous_page in pages.iter() {
                        let prev_rule = rules.get(previous_page).unwrap_or(&empty);
                        // println!("{}?{} {}|{:?}",previous_page, page, previous_page, prev_rule);
                        if prev_rule.contains(&page) {
                            println!("BAD");
                            bad = true;
                            break 'page;
                        }
                    }
                    pages.push(*page);
                }
                if bad {
                    _pages.sort_by(|a, b| {
                        Page {
                            value: *a,
                            before: rules.get(a).unwrap_or(&empty).to_vec(),
                        }
                        .partial_cmp(&Page {
                            value: *b,
                            before: rules.get(b).unwrap_or(&empty).to_vec(),
                        })
                        .unwrap()
                    });
                    let middle = _pages[_pages.len() / 2];
                    println!("{}", middle);
                    acc += middle;
                }
            }
        }
        println!("Value: {}", acc);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn page_ordering() {
            let page1 = Page {
                value: 10,
                before: vec![2, 3, 5],
            };

            let page2 = Page {
                value: 2,
                before: vec![1],
            };

            let page3 = Page {
                value: 3,
                before: vec![2],
            };

            assert!(page2 < page1);

            let mut pages = vec![&page1, &page2];

            pages.sort_by(|a, b| a.partial_cmp(b).unwrap());

            assert!(pages == vec![&page2, &page1]);

            let mut pages = vec![&page1, &page2, &page3];

            pages.sort_by(|a, b| a.partial_cmp(b).unwrap());

            assert!(pages == vec![&page2, &page3, &page1]);
        }
    }
}

pub mod aoc_2024_06 {
    use std::io::BufRead;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Map {
        Obstacle,
        Visited,
        NotVisited,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub enum Guard {
        N,
        E,
        S,
        W,
    }

    pub fn aoc_2024_06_01() {
        // let file_path = "data/2024/06-example.txt";
        let file_path = "data/2024/06.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let reader = std::io::BufReader::new(file);

        let mut map: Vec<Vec<Map>> = Vec::new();

        let mut guard_pos = (0i32, 0i32);
        let mut dir = Guard::N;

        for (_row, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
            let mut row = vec![];
            for (col, c) in line.chars().enumerate() {
                match c {
                    '.' => row.push(Map::NotVisited),
                    '#' => row.push(Map::Obstacle),
                    '^' => {
                        row.push(Map::Visited);
                        guard_pos = (_row as i32, col as i32);
                        dir = Guard::N;
                    }
                    _ => unreachable!(),
                }
            }
            map.push(row);
        }

        let height = map.len() as i32;
        let width = map[0].len() as i32;

        loop {
            let new_pos: (i32, i32);
            match dir {
                Guard::N => new_pos = (guard_pos.0 - 1, guard_pos.1),
                Guard::S => new_pos = (guard_pos.0 + 1, guard_pos.1),
                Guard::E => new_pos = (guard_pos.0, guard_pos.1 + 1),
                Guard::W => new_pos = (guard_pos.0, guard_pos.1 - 1),
            }

            // Escaped
            if new_pos.0 < 0 || new_pos.0 >= height || new_pos.1 < 0 || new_pos.1 >= width {
                break;
            }

            // Step
            match &map[new_pos.0 as usize][new_pos.1 as usize] {
                Map::Visited => guard_pos = new_pos,
                Map::NotVisited => {
                    guard_pos = new_pos;
                    map[new_pos.0 as usize][new_pos.1 as usize] = Map::Visited
                }
                // Don't move, just turn
                Map::Obstacle => match dir {
                    Guard::N => dir = Guard::E,
                    Guard::E => dir = Guard::S,
                    Guard::S => dir = Guard::W,
                    Guard::W => dir = Guard::N,
                },
            }
        }
        println!(
            "Visited: {}",
            map.iter()
                .flatten()
                .filter(|spot| **spot == Map::Visited)
                .count()
        );
    }

    pub enum WalkResult {
        Escape(u32),
        Cycle,
    }

    pub fn walk(mut map: Vec<Vec<Map>>, start_pos: (i32, i32), start_dir: Guard) -> WalkResult {
        let mut guard_pos = start_pos.clone();
        let mut dir = start_dir.clone();

        let height = map.len() as i32;
        let width = map[0].len() as i32;

        let mut nodes: std::collections::HashSet<((i32, i32), Guard)> =
            std::collections::HashSet::new();

        loop {
            let new_pos: (i32, i32);
            match dir {
                Guard::N => new_pos = (guard_pos.0 - 1, guard_pos.1),
                Guard::S => new_pos = (guard_pos.0 + 1, guard_pos.1),
                Guard::E => new_pos = (guard_pos.0, guard_pos.1 + 1),
                Guard::W => new_pos = (guard_pos.0, guard_pos.1 - 1),
            }

            // Escaped
            if new_pos.0 < 0 || new_pos.0 >= height || new_pos.1 < 0 || new_pos.1 >= width {
                break;
            }

            // Step
            match &map[new_pos.0 as usize][new_pos.1 as usize] {
                Map::Visited => guard_pos = new_pos,
                Map::NotVisited => {
                    guard_pos = new_pos;
                    map[new_pos.0 as usize][new_pos.1 as usize] = Map::Visited
                }
                // Don't move, just turn
                Map::Obstacle => {
                    let node = (new_pos, dir.clone());
                    if nodes.contains(&node) {
                        return WalkResult::Cycle;
                    }
                    nodes.insert(node);
                    match dir {
                        Guard::N => dir = Guard::E,
                        Guard::E => dir = Guard::S,
                        Guard::S => dir = Guard::W,
                        Guard::W => dir = Guard::N,
                    }
                }
            }
        }

        return WalkResult::Escape(
            map.iter()
                .flatten()
                .filter(|spot| **spot == Map::Visited)
                .count() as u32,
        );
    }

    pub fn aoc_2024_06_02() {
        // let file_path = "data/2024/06-example.txt";
        let file_path = "data/2024/06.txt";
        let file = std::fs::File::open(file_path).expect("file wasn't found.");
        let reader = std::io::BufReader::new(file);

        let mut map: Vec<Vec<Map>> = Vec::new();

        let mut guard_pos = (0i32, 0i32);
        let mut dir = Guard::N;

        for (_row, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
            let mut row = vec![];
            for (col, c) in line.chars().enumerate() {
                match c {
                    '.' => row.push(Map::NotVisited),
                    '#' => row.push(Map::Obstacle),
                    '^' => {
                        row.push(Map::Visited);
                        guard_pos = (_row as i32, col as i32);
                        dir = Guard::N;
                    }
                    _ => unreachable!(),
                }
            }
            map.push(row);
        }

        let mut cycles = 0u32;

        for row in 0..map.len() {
            for col in 0..map[0].len() {
                match map[row][col] {
                    Map::NotVisited => {
                        let mut check_map = map.clone();
                        check_map[row][col] = Map::Obstacle;
                        match walk(check_map, guard_pos, dir) {
                            WalkResult::Cycle => cycles += 1,
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
        }

        println!("Cycles: {}", cycles);
    }
}
