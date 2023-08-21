pub mod task_3 {
    use read_file::read_file;

    fn is_nth_eq(line: &str, i: usize, c: char) -> bool {
        line.chars().nth(i).expect("exist") == c
    }

    fn find_rating(lines: Vec<String>, compare: fn(x: i32, y: i32) -> bool) -> i32 {
        let len = &lines.first().expect("file wasn't read").len();
        let mut new_lines = lines.iter().collect::<Vec<&String>>();
        for i in 0..*len {
            if new_lines.len() == 1 {
                break;
            }
            let mut count = vec![0; 2];
            for line in &new_lines {
                let bit = if is_nth_eq(line, i, '0') { 0 } else { 1 };
                count[bit] += 1;
            }
            let check_char = match compare(count[0], count[1]) {
                true => '0',
                false => '1',
            };
            new_lines = new_lines
                .into_iter()
                .filter(|line| is_nth_eq(line, i, check_char))
                .collect::<Vec<&String>>()
        }
        // println!("RES:{:?}", new_lines);
        i32::from_str_radix(new_lines.first().unwrap(), 2).unwrap()
    }

    pub fn task_3_1() {
        let lines = read_file("data_files/file3.txt");
        let len = lines.first().expect("file wasn't read").len();
        let mut count = vec![[0, 0]; len];
        for line in lines {
            for (i, c) in line.chars().enumerate() {
                let bit = if c == '0' { 0 } else { 1 };
                count[len - i - 1][bit] += 1;
            }
        }
        let value: u32 = count
            .iter()
            .map(|[zeros, ones]| if zeros > ones { 0 } else { 1 })
            .fold(0, |mut acc, x| {
                acc <<= 1;
                acc | x
            });
        println!("{:?}", (value * (!value & 0x0FFF)));
    }

    pub fn task_3_2() {
        let lines = read_file("data_files/file3.txt");
        let oxygen = find_rating(lines.clone(), |zeros, ones| zeros > ones);
        let co2 = find_rating(lines, |zeros, ones| zeros <= ones);
        println!("{}", oxygen * co2)
    }
}
