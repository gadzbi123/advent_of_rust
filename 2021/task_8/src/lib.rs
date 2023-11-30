use read_file::read_file;
use std::cell::RefCell;
use std::collections::HashMap;
fn task_8_1() {
    let lines = read_file("./data_files/file8.txt");
    let mut count = 0;
    for line in lines {
        let (config, displayed_str) = line.split_once('|').unwrap();
        let displayed_selected: Vec<&str> = displayed_str
            .split(' ')
            .filter(|&x| x.len().eq(&2) || x.len().eq(&3) || x.len().eq(&4) || x.len().eq(&7))
            .collect();
        count += displayed_selected.len();
    }
    dbg!(count);
}

#[derive(Debug, Clone)]
struct SevenSegmentStr {
    value: String,
}
impl SevenSegmentStr {
    fn new(value: String) -> Self {
        Self { value }
    }
    fn update(&mut self, value: String) {
        self.value = value;
    }
    fn is_same(&self, rhs: &str) -> bool {
        if self.value.len() != rhs.len() {
            return false;
        }
        for c in self.value.chars() {
            if rhs.find(c).is_none() {
                return false;
            }
        }
        true
    }
}

fn task_8_2_1() {
    let lines: Vec<String> = read_file("./data_files/file8.txt");
    let digits: Vec<RefCell<SevenSegmentStr>> =
        vec![RefCell::new(SevenSegmentStr::new("".to_string())); 10];
    let mut sum = 0;
    for line in lines {
        let (config_str, displayed_numbers_str) = line.split_once('|').unwrap();
        let config: Vec<String> = config_str
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect();
        //algo
        digits[8]
            .borrow_mut()
            .update(config.iter().find(|x| x.len() == 7).unwrap().to_string());
        digits[7]
            .borrow_mut()
            .update(config.iter().find(|x| x.len() == 3).unwrap().to_string());
        digits[4]
            .borrow_mut()
            .update(config.iter().find(|x| x.len() == 4).unwrap().to_string());
        digits[1]
            .borrow_mut()
            .update(config.iter().find(|x| x.len() == 2).unwrap().to_string());
        let mut zero_6_9: Vec<_> = config.iter().filter(|x| x.len() == 6).collect();
        let mut index_drop = 3;
        //find 6
        for c in digits[1].borrow().value.chars() {
            for (i, digit) in zero_6_9.iter().enumerate() {
                if digit.find(c).is_none() {
                    digits[6].borrow_mut().update(digit.to_string());
                    index_drop = i;
                }
            }
        }
        zero_6_9.remove(index_drop);
        //find 0 and 9
        let zero_9 = zero_6_9;
        for c in digits[4].borrow().value.chars() {
            for (i, num) in zero_9.iter().enumerate() {
                if num.find(c).is_none() {
                    digits[0].borrow_mut().update(zero_9[i].to_string());
                    digits[9]
                        .borrow_mut()
                        .update(zero_9[(i == 0) as usize].to_string());
                }
            }
        }
        //find 3
        let mut two_3_5: Vec<_> = config.iter().filter(|x| x.len() == 5).collect();
        let mut two_5: Vec<String> = Vec::new();
        for c in digits[1].borrow().value.chars() {
            let mut to_remove = 3;
            for (i, word) in two_3_5.iter().enumerate() {
                if word.find(c).is_none() {
                    to_remove = i;
                    break;
                }
            }
            two_5.push(two_3_5.remove(to_remove).to_string());
        }
        digits[3].borrow_mut().update(two_3_5[0].to_string());
        //find 2 and 5
        let mut did_change = false;
        for c in two_5[0].chars() {
            if digits[9].borrow().value.find(c).is_none() {
                did_change = true;
                digits[2].borrow_mut().update(two_5[0].to_string());
                digits[5].borrow_mut().update(two_5[1].to_string());
            }
        }
        if !did_change {
            digits[2].borrow_mut().update(two_5[1].to_string());
            digits[5].borrow_mut().update(two_5[0].to_string());
        }
        let displayed_numbers: Vec<&str> = displayed_numbers_str
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect();

        let mut result = 0;
        for number in displayed_numbers {
            for (i_digit, digit) in digits.iter().enumerate() {
                if digit.borrow().is_same(number) {
                    result = result * 10 + i_digit;
                    break;
                }
            }
        }
        sum += result;
    }
    dbg!(sum);
}

//   aaaa
//  f    b
//  f    b
//   gggg
//  e    c
//  e    c
//   dddd
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}
impl Segment {
    fn encode(c: char) -> Segment {
        match c {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            _ => unreachable!(),
        }
    }
    fn decode(s: Segment) -> char {
        match s {
            Segment::A => 'a',
            Segment::B => 'b',
            Segment::C => 'c',
            Segment::D => 'd',
            Segment::E => 'e',
            Segment::F => 'f',
            Segment::G => 'g',
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct SevenSegment(HashMap<Segment, Segment>);
impl SevenSegment {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn insert(&mut self, k: Segment, v: Segment) {
        self.0.insert(k, v);
    }
    fn get(&self, k: &Segment) -> &Segment {
        self.0.get(k).unwrap()
    }
    fn has_value(&self, s: Segment) -> bool {
        self.0.iter().any(|(_, &v)| v == s)
    }
    fn find_key_of_value(&self, s: Segment) -> Segment {
        *self.0.iter().find(|(&k, &v)| v == s).unwrap().0
    }
    fn clear(&mut self) {
        self.0.clear();
    }
    fn as_u32(&self) -> u32 {
        match self.0.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 => {
                if self.0.contains_key(&Segment::E) {
                    return 2;
                }
                if self.0.contains_key(&Segment::F) {
                    return 5;
                }
                3
            }
            6 => {
                if !self.0.contains_key(&Segment::G) {
                    return 0;
                }
                if !self.0.contains_key(&Segment::B) {
                    return 6;
                }
                9
            }
            7 => 8,
            _ => unreachable!(),
        }
    }
}
fn task_8_2_2() {
    let lines = read_file("./data_files/file8.txt");
    let mut seven_segment_pattern = SevenSegment::new();
    let mut sum = 0;
    for line in lines {
        let (config_str, displayed_numbers_str) = line.split_once('|').unwrap();
        let config: Vec<&str> = config_str.split(' ').filter(|x| !x.is_empty()).collect();
        let seven = *config.iter().find(|x| x.len() == 3).unwrap();
        let one = *config.iter().find(|x| x.len() == 2).unwrap();
        //A
        for segment in seven.chars() {
            if one.find(|c| c == segment).is_none() {
                seven_segment_pattern.insert(Segment::A, Segment::encode(segment));
            }
        }
        //B
        let mut zero_6_9: Vec<_> = config.iter().filter(|x| x.len() == 6).collect();
        let mut index_drop = 3;
        let mut six = None;
        for c in one.chars() {
            for (i, &digit) in zero_6_9.iter().enumerate() {
                if digit.find(c).is_none() {
                    six = Some(digit.to_string());
                    index_drop = i;
                }
            }
        }
        zero_6_9.remove(index_drop);
        let mut zero_9 = zero_6_9;
        for segment in one.chars() {
            if six
                .clone()
                .expect("Be six")
                .find(|c| c == segment)
                .is_none()
            {
                seven_segment_pattern.insert(Segment::B, Segment::encode(segment))
            }
        }
        //C
        for c in one.chars() {
            if seven_segment_pattern.get(&Segment::B) != &Segment::encode(c) {
                seven_segment_pattern.insert(Segment::C, Segment::encode(c));
            }
        }
        //G
        let four = config.iter().find(|x| x.len() == 4).unwrap();
        for c in four.chars() {
            for (i, &digit) in zero_9.iter().enumerate() {
                if digit.find(|x| x == c).is_none() {
                    seven_segment_pattern.insert(Segment::G, Segment::encode(c));
                    index_drop = i;
                }
            }
        }
        //E
        zero_9.remove(index_drop);
        let nine = *zero_9.first().unwrap();
        let eight = config.iter().find(|x| x.len() == 7).unwrap();
        for c in eight.chars() {
            if nine.find(|ch| ch == c).is_none() {
                seven_segment_pattern.insert(Segment::E, Segment::encode(c));
            }
        }
        //F
        for c in four.chars() {
            let current_segment = Segment::encode(c);
            if !seven_segment_pattern.has_value(current_segment) {
                seven_segment_pattern.insert(Segment::F, current_segment);
            }
        }
        //D
        for c in eight.chars() {
            let current_segment = Segment::encode(c);
            if !seven_segment_pattern.has_value(current_segment) {
                seven_segment_pattern.insert(Segment::D, current_segment);
            }
        }
        let displayed_numbers: Vec<&str> = displayed_numbers_str
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect();
        let mut number = 0;
        for number_str in displayed_numbers {
            let mut digit = SevenSegment::new();
            for c in number_str.chars() {
                digit.insert(
                    seven_segment_pattern.find_key_of_value(Segment::encode(c)),
                    Segment::encode(c),
                )
            }
            number = number * 10 + digit.as_u32();
        }
        sum += number;
        // seven_segment_pattern.clear();
    }
    dbg!(sum);
}

pub fn task_8() {
    task_8_1();
    let first = std::time::SystemTime::now();
    task_8_2_1();
    dbg!(first.elapsed().ok().unwrap());
    let second = std::time::SystemTime::now();
    task_8_2_2();
    dbg!(second.elapsed().ok().unwrap());
}
