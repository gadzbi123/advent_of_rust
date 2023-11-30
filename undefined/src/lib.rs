use core::num;
use std::{
    cell::{Cell, RefCell},
    collections::{binary_heap, BinaryHeap, HashMap},
    ops::Deref,
    process::exit,
    rc::Rc,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
    time::{Duration, Instant},
    usize,
};

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    value: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value)
    }
}

unsafe fn unsafe_rust() {
    let mem: Vec<u32> = vec![1, 3];
    let mut first_half_addr = mem.as_ptr() as *mut u16;
    println!("{:?}", first_half_addr);
    for _ in 0..4 {
        *first_half_addr += 1;
        println!("{:?}", *(first_half_addr));
        first_half_addr = first_half_addr.offset(1);
    }
    println!("{:032b}\n{:032b}", mem[0], mem[1]);
}

pub fn mutate_grid() {
    let mut grid = [[0_u8; 5]; 5];
    for row in grid.iter_mut() {
        for (i, item) in row.iter_mut().enumerate() {
            if i == 2 {
                *item = 1;
            }
            print!("{} ", item);
        }
        println!();
    }
}

pub fn mutate_array_cell() {
    let vec = [Cell::new(5), Cell::new(25)];
    let first = &vec[0];
    let second = &vec[1];
    first.set(second.get());
    dbg!(vec);
}

pub fn mutate_array_ref_cell() {
    let vec = [(RefCell::new(5)), (RefCell::new(25))];

    {
        let mut x = vec[0].borrow_mut();
        let y = vec[1].borrow();
        *x = *y;
        dbg!(&vec); //cant see vec[0] (borrowed)
    }
    dbg!(&vec); // not borrowed, can see all vec

    *vec[1].borrow_mut() = 10;
    dbg!(&vec);

    vec[0].swap(&vec[1]);
    dbg!(&vec);
}

#[derive(Debug, Clone, Copy)]
struct my_struct<'a> {
    number: i32,
    name: &'a str,
}

pub fn mutate_array_ref_cell_struct_str() {
    let vec = [
        RefCell::new(my_struct {
            number: 5,
            name: "ala",
        }),
        (RefCell::new(my_struct {
            number: 10,
            name: "kacper",
        })),
    ];

    {
        let mut x = vec[0].borrow_mut();
        let y = vec[1].borrow();
        *x = *y;
        dbg!(&vec); //cant see vec[0] (borrowed)
    }
    dbg!(&vec); // not borrowed, can see all vec

    vec[1].borrow_mut().name = "tomek";
    dbg!(&vec);

    vec[0].swap(&vec[1]);
    dbg!(&vec);
}

pub fn mutate_array_rc() {
    let mut vec = [(Rc::new(RefCell::new(5))), Rc::new(RefCell::new(25))]; //dont mutate vec

    let mut x = Rc::clone(&vec[0]); //dont mutate a ref
    let y = Rc::clone(&vec[1]);
    *x.borrow_mut() = *y.borrow();
    dbg!(&vec);

    Rc::clone(&vec[1]).replace(10);
    dbg!(&vec);

    Rc::clone(&vec[0]).swap(&Rc::clone(&vec[1]));
    dbg!(&vec);
}

// A_G multiply problem
// TASK: find G if every different letter is different digit
// ABCDE*F=GGGGGG
// Overview
//  ABCDE
// *    F
// GGGGGG
pub fn find_g() {
    let mut results: Vec<_> = Vec::new();
    for g in 1..=9 {
        let G = (g * 111_111) as f64;
        for f in 2..=9 {
            let F = f as f64;
            let A_TO_E = G / F;
            if A_TO_E % 1. == 0. {
                let mut digits = Vec::new();
                let mut a_to_e = A_TO_E as i32;
                while a_to_e != 0 {
                    let last_digit = a_to_e % 10;
                    if !digits.iter().any(|&x| x == last_digit) {
                        digits.push(last_digit);
                    }
                    a_to_e /= 10;
                }
                if digits.len() == 5 {
                    // dbg!(A_TO_E, F, G);
                    results.push(G as i32 % 10);
                }
            }
        }
    }
    dbg!(results);
}

// async
pub fn threads_with_channel() {
    fn do_stuff(tx: Sender<u8>, n: u8) {
        tx.send(n).unwrap();
    }
    let (tx_orginal, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let tx = tx_orginal.clone();
    thread::spawn(|| do_stuff(tx, 1));
    let tx = tx_orginal.clone();
    thread::spawn(|| do_stuff(tx, 2));
    let tx = tx_orginal.clone();
    thread::spawn(|| do_stuff(tx, 3));
    while let Ok(r) = rx.recv() {
        dbg!(r);
    }
}
// variable output: 1, 2, 3 or 3, 1, 2 }

//Rc<RefCell<T>>
pub type todo_type = Rc<RefCell<Todo>>;
pub type todos_type = Vec<todo_type>;

#[derive(Debug)]
pub struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

impl Todo {
    pub fn new(text: String, id: usize) -> Rc<RefCell<Todo>> {
        Rc::from(RefCell::new(Todo {
            text,
            id,
            completed: false,
        }))
    }
}

pub struct Todos {
    todos: todos_type,
}

impl Todos {
    pub fn new(todos: todos_type) -> Todos {
        Todos { todos }
    }

    pub fn get_all_todos(self) -> todos_type {
        self.todos
    }

    pub fn get_single_todo(self, todo_index: usize) -> Todo {
        unimplemented!()
    }

    pub fn add_todo(&mut self, text: String) -> todo_type {
        let id: usize = 1;

        if self.todos.is_empty() {
            let id = 1;
        } else {
            let last_todo = match self.todos.len() {
                0 => None,
                n => Some(&self.todos[n - 1]),
            };
            let id = last_todo.unwrap().borrow().id;
        }
        let todo = Todo::new(text, id);
        self.todos.push(todo.to_owned());
        Rc::clone(&todo)
    }

    pub fn remove_todo(mut self, todo_index: usize) -> bool {
        self.todos.remove(todo_index);

        true
    }
}

pub fn rc_refcell_shared_mut() {
    let mut todos = Todos::new(vec![Todo::new("abc".to_string(), 1)]);
    // we have own pointer to RefCell, need to borrow when we use it
    let my_todo: Rc<RefCell<Todo>> = todos.add_todo("dfg".to_string());
    // we have a ref to Rc, don't need to borrow every time
    let last_todo: &Rc<RefCell<Todo>> = todos.todos.last().unwrap();
    dbg!(last_todo);
    dbg!(my_todo.borrow());
    my_todo.borrow_mut().id = 5;
    dbg!(my_todo.borrow());
    dbg!(last_todo);
    // here we triple dereference a pointer, from a ref given -> from a Rc -> from RefCell
    last_todo.borrow_mut().text = "xdd".to_string();
    // syntactic sugar will help us out with it so it is more readable
    // What actually happens is this:
    // (**last_todo).borrow_mut().text = "xdd".to_string();
    dbg!(my_todo);
    dbg!(last_todo);
}

pub fn my_atoi(num_str: String) -> i32 {
    let mut integer = 0i32;
    if num_str.is_empty() {
        return integer;
    }
    let num_start = num_str.find(|x: char| x.is_ascii_digit());
    if num_start.is_none() {
        return integer;
    }
    let ascii_no_allowed = num_str.find(|x: char| x.is_ascii() && x != ' ' && x != '-' && x != '+');
    if let Some(ans) = ascii_no_allowed {
        if num_start.unwrap() > ans {
            return integer;
        }
    }
    let sign = num_str.find(|x: char| x == '-' || x == '+');
    if let Some(sign_index) = sign {
        if num_start.unwrap() != sign_index + 1 && sign_index < num_start.unwrap() {
            return integer;
        }
    }
    let is_negative = sign.is_some()
        && num_str.as_bytes()[sign.unwrap()] as char == '-'
        && num_start.unwrap() == sign.unwrap() + 1;
    for &c in num_str
        .as_bytes()
        .iter()
        .take(num_str.len())
        .skip(num_start.unwrap())
    {
        match (c as char).to_digit(10) {
            Some(digit) => {
                integer = integer.saturating_mul(10);
                if integer < 0 {
                    integer = integer.saturating_sub(digit as i32);
                    continue;
                }
                integer = integer.saturating_add(digit as i32);
                if integer != 0 && is_negative {
                    integer *= -1;
                }
            }
            None => {
                return integer;
            }
        }
    }
    integer
}

#[test]
fn my_atoi_success() {
    assert_eq!(my_atoi("-13+8".to_string()), -13);
    assert_eq!(my_atoi("".to_string()), 0);
    assert_eq!(my_atoi("+".to_string()), 0);
    assert_eq!(my_atoi("123".to_string()), 123);
    assert_eq!(my_atoi("-123".to_string()), -123);
    assert_eq!(my_atoi("123-".to_string()), 123);
    assert_eq!(my_atoi("     123 i like shit".to_string()), 123);
    assert_eq!(my_atoi("    -123".to_string()), -123);
    assert_eq!(my_atoi("    -123 i like shit".to_string()), -123);
    assert_eq!(my_atoi("    +123 i like shit".to_string()), 123);
    assert_eq!(my_atoi("+123 i like shit".to_string()), 123);
    assert_eq!(my_atoi("-123 0".to_string()), -123);
    assert_eq!(my_atoi("+-123".to_string()), 0);
    assert_eq!(my_atoi("-+123".to_string()), 0);
    assert_eq!(my_atoi("words and 987".to_string()), 0);
}

pub fn binary_vs_normal_search() {
    let mut step = 10;
    let search_times = 100;
    while step <= 1_000_000_000 {
        let mut rng = rand::thread_rng();
        let mut array: Vec<usize> = (0..step).collect();
        array.shuffle(&mut rng);
        let mut nums_to_find = Vec::with_capacity(search_times);
        for _ in 0..search_times {
            nums_to_find.push(rng.gen_range(0..step))
        }
        let mut first_result = Vec::with_capacity(search_times);
        let mut second_result = Vec::with_capacity(search_times);
        let mut is_sorted = false;
        for num_to_find in nums_to_find.iter().take(search_times) {
            let first = Instant::now();
            array.iter().position(|x| x == num_to_find);
            first_result.push(first.elapsed().as_micros() as usize);
            let second = Instant::now();
            if !is_sorted {
                array.sort();
                is_sorted = true;
            }
            let _ = array.binary_search(num_to_find);
            second_result.push(second.elapsed().as_micros() as usize);
        }
        println!(
            "{} -> , avg: {:?}us, {:?}us",
            step,
            first_result.iter().sum::<usize>() / search_times,
            second_result.iter().sum::<usize>() / search_times,
        );
        first_result.sort();
        second_result.sort();
        println!(
            "{} -> , med: {:?}us, {:?}us",
            step,
            first_result[search_times / 2],
            second_result[search_times / 2]
        );
        println!(
            "{} -> , mode: {:?}us, {:?}us",
            step,
            first_result
                .iter()
                .fold(HashMap::new(), |mut acc, res| {
                    acc.insert(res, *acc.get(res).unwrap_or(&0) + 1);
                    acc
                })
                .iter()
                .fold((0, 0), |acc, (&&key, &value)| {
                    if acc.1 < (value) {
                        (key, value)
                    } else {
                        acc
                    }
                })
                .0,
            second_result
                .iter()
                .fold(HashMap::new(), |mut acc, res| {
                    acc.insert(res, *acc.get(res).unwrap_or(&0) + 1);
                    acc
                })
                .iter()
                .fold((0, 0), |acc, (&&key, &value)| {
                    if acc.1 < (value) {
                        (key, value)
                    } else {
                        acc
                    }
                })
                .0,
        );
        step *= 10;
    }
}

fn next_permutation(array: &mut [i32], n: usize) {
    if n == 1 {
        return;
    }
    for i in 0..n {
        next_permutation(array, n - 1);
        if n % 2 == 0 {
            array.swap(i, n - 1);
        } else {
            array.swap(0, n - 1);
        }
    }
}
// This algorith can take from nanoseconds to the end of the universe
pub fn bogo_sort() {
    const range: i32 = 10;
    let mut array: Vec<i32> = (0..range).collect();
    next_permutation(array.as_mut_slice(), 2);
    println!("Starting a bogo sort algorith");
    let time1 = Instant::now();
    while !array.windows(2).all(|w| w[0] <= w[1]) {
        let random_perm = thread_rng().gen_range(2..range - 1) as usize;
        next_permutation(array.as_mut_slice(), random_perm);
    }
    let time_result = time1.elapsed();
    println!(
        "Manged to sort array after {}s {}ms-> {:?}",
        time_result.as_secs(),
        time_result.subsec_millis(),
        array
    );
}
