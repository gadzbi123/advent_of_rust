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

fn mutate_grid() {
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
