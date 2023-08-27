use std::{
    cell::{Cell, RefCell},
    rc::Rc,
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
