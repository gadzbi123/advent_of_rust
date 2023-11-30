use read_file::read_file;
use std::cell::RefCell;
use std::sync::atomic::Ordering;
use std::{
    collections::{HashMap, HashSet},
    sync::atomic::AtomicUsize,
    time::Instant,
};

static TOTAL_CALLS: AtomicUsize = AtomicUsize::new(0);
pub fn task_13() {
    let file = read_file("./data_files/file13.txt");
    let mut field = Field::new();
    for line in file {
        match line.split_once(',') {
            Some((x, y)) => mark_field(&mut field, x, y),
            None => try_fold_field(line, &mut field),
        }
    }
    // dbg!(&marked_fields);
    draw_field(&field);
}
fn mark_field(field: &mut Field, x: &str, y: &str) {
    let position = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
    field.width = field.width.max(position.0 + 1);
    field.height = field.height.max(position.1 + 1);
    field.marked_fields.borrow_mut().insert(position);
}

#[derive(Debug)]
enum Direction {
    X,
    Y,
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "x" => Direction::X,
            "y" => Direction::Y,
            err => panic!("No direction {err}"),
        }
    }
}
#[derive(Debug)]
struct Field {
    marked_fields: RefCell<HashSet<(usize, usize)>>,
    width: usize,
    height: usize,
}
impl Field {
    fn new() -> Self {
        Field {
            marked_fields: RefCell::from(HashSet::new()),
            width: 0,
            height: 0,
        }
    }
}
fn try_fold_field(line: String, field: &mut Field) {
    let fold_values: String = line.chars().skip("fold along ".len()).collect();

    if let Some((direction, fold_index)) = fold_values.split_once('=') {
        TOTAL_CALLS.fetch_add(1, Ordering::SeqCst);
        fold_field(field, direction.into(), fold_index.parse().unwrap());
        if TOTAL_CALLS.load(Ordering::SeqCst) == 1 {
            println!("Marked fields = {}", field.marked_fields.borrow().len())
        }
    }
}

fn fold_field(field: &mut Field, direction: Direction, fold_index: usize) {
    match direction {
        Direction::X => field.width /= 2,
        Direction::Y => field.height /= 2,
    };
    let fields = field.marked_fields.borrow();
    let folded_side_fields: HashSet<(usize, usize)> = fields
        .iter()
        .filter(|(x, y)| match direction {
            Direction::X => *x > fold_index,
            Direction::Y => *y > fold_index,
        })
        .cloned()
        .collect();
    drop(fields);
    folded_side_fields.iter().for_each(|(x, y)| {
        let new_value = match direction {
            Direction::X => (2 * fold_index - x, *y),
            Direction::Y => (*x, 2 * fold_index - y),
        };
        field.marked_fields.borrow_mut().insert(new_value);
    });

    field
        .marked_fields
        .borrow_mut()
        .retain(|(x, y)| match direction {
            Direction::X => *x < fold_index,
            Direction::Y => *y < fold_index,
        })
}

fn draw_field(field: &Field) {
    for y in 0..field.height {
        for x in 0..field.width {
            match field.marked_fields.borrow().contains(&(x, y)) {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
    println!();
}
