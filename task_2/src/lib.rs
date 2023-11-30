use read_file::read_file;
fn task_2_1() {
    let lines = read_file("./src/file2.txt");
    let mut point: (i32, i32) = (0, 0);
    for line in lines {
        let line_vec = line.split(' ').collect::<Vec<&str>>();
        let x = line_vec[1].parse::<i32>().expect("i32");
        match line_vec[0] {
            "forward" => point.0 += x,
            "up" => point.1 += x,
            "down" => point.1 -= x,
            _ => (),
        }
    }
    println!("{:?}", point.0 * point.1.abs())
}
fn task_2_2() {
    let lines = read_file("./src/file2.txt");
    let mut point: (i32, i32) = (0, 0);
    let mut aim = 0;
    for line in lines {
        let line_vec = line.split(' ').collect::<Vec<&str>>();
        match line_vec[..] {
            ["forward", str_value] => {
                let value = str_value.parse::<i32>().expect("i32");
                point.0 += value;
                point.1 += value * aim
            }
            ["down", str_value] => aim += str_value.parse::<i32>().expect("i32"),
            ["up", str_value] => aim -= str_value.parse::<i32>().expect("i32"),
            _ => (),
        }
    }
    println!("{:?}", point.0 * point.1.abs())
}

pub fn task_2() {
    task_2_1();
    task_2_2();
}
