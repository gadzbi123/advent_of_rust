// trait FunctionFormula {
//     fn x(f32, &i32) -> f32,
// }

type FunctionFormula = dyn FnMut(f32, &i32) -> f32;
fn formula_of_task(task: i32, middle: i32) -> Box<FunctionFormula> {
    match task {
        1 => Box::new(move |acc: f32, x: &i32| acc + (middle - x).abs() as f32),
        2 => Box::new(move |acc: f32, x: &i32| {
            // Arithmetic sum formula
            let len = (middle - *x).abs() as f32;
            acc + ((1.0 + len) / 2.0 * len)
        }),
        _ => unreachable!(),
    }
}

fn task(NUMBER: i32) {
    let mut vec: Vec<i32> = read_file("./data_files/file7.txt")
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    vec.sort();
    let range = vec[0]..=vec[vec.len() - 1];
    let mut result = f32::MAX;
    for middle in range {
        result = vec
            .iter()
            .fold(0.0, formula_of_task(NUMBER, middle))
            .min(result);
    }
    dbg!(result as i32);
}
pub fn task_7() {
    task(1);
    task(2);
}
