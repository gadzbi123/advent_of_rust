use std::{
    cell::{Cell, RefCell},
    ops::Deref,
    process::exit,
    rc::Rc,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
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
