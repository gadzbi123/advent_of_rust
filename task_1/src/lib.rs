use read_file::read_file;

fn task_1_1() {
    let lines = read_file("./src/file1.txt");
    let mut count = 0;
    let mut previous: &String = lines.first().unwrap();
    for line in lines.iter() {
        match line {
            line if line.parse::<i32>().unwrap() > previous.parse::<i32>().unwrap() => count += 1,
            _ => (),
        };
        previous = line;
    }
    println!("{}", count)
}

fn task_1_2() {
    let lines = read_file("./src/file1.txt");
    let mut prev_sum: Option<u32> = None;
    let mut count = 0;
    for n in 0..lines.len() - 2 {
        let sum = &lines[n..=n + 2]
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .sum::<u32>();

        if prev_sum.is_some_and(|x| &x < sum) {
            count += 1;
        }
        prev_sum = Some(*sum);
    }
    println!("{}", count);
}
