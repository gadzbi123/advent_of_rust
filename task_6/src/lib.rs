pub fn task_6() {
    task_6_1();
    task_6_2();
}

fn task_6_1() {
    let lines = read_file::read_file("./data_files/file6.txt");
    let mut fishes: Vec<u32> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let days = 80;
    for _ in 0..days {
        let mut increase_count = 0;
        fishes = fishes
            .clone()
            .iter()
            .map(|f| {
                *f.checked_sub(1).get_or_insert_with(|| {
                    increase_count += 1;
                    6
                })
            })
            .collect();
        fishes.append(&mut vec![8_u32; increase_count]);
        // dbg!(&fishes);
    }
    dbg!(fishes.len());
}

fn task_6_2() {
    let lines = read_file::read_file("./data_files/file6.txt");
    let mut fishes: HashMap<u32, u64> = HashMap::new();
    lines.first().unwrap().split(',').for_each(|fish_str| {
        let fish = fish_str.parse::<u32>().unwrap();
        let amount = fishes.get(&fish).unwrap_or(&0) + 1;
        fishes.insert(fish, amount);
    });
    const DAYS: i32 = 256;
    for _ in 0..DAYS {
        let mut fishes_clone = fishes.clone();
        for fish_day in (0..=8).rev() {
            if fish_day == 0 {
                let birth_fish = *fishes.get(&0).unwrap_or(&0);
                fishes_clone.insert(6, fishes.get(&7).unwrap_or(&0) + birth_fish);
                fishes_clone.insert(8, fishes.get(&9).unwrap_or(&0) + birth_fish);
            }
            fishes_clone.insert(fish_day, *fishes.get(&(fish_day + 1)).unwrap_or(&0));
        }
        fishes = fishes_clone;
    }
    let count = fishes.iter().fold(0, |acc, (_, v)| acc + v);
    dbg!(count);
}
