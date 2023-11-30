use read_file::read_file_bytes;

#[derive(Debug, Clone, Copy)]
enum BracketScores {
    Parentheses,
    Square,
    Curly,
    Angled,
}

impl From<Brackets> for BracketScores {
    fn from(value: Brackets) -> Self {
        match value {
            Brackets::LParentheses | Brackets::RParentheses => BracketScores::Parentheses,
            Brackets::LSquare | Brackets::RSquare => BracketScores::Square,
            Brackets::LCurly | Brackets::RCurly => BracketScores::Curly,
            Brackets::LAngled | Brackets::RAngled => BracketScores::Angled,
        }
    }
}

impl BracketScores {
    fn value1(&self) -> i32 {
        use BracketScores::*;
        match self {
            Parentheses => 3,
            Square => 57,
            Curly => 1197,
            Angled => 25137,
        }
    }
    fn value2(&self) -> i64 {
        use BracketScores::*;
        match self {
            Parentheses => 1,
            Square => 2,
            Curly => 3,
            Angled => 4,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Brackets {
    LParentheses,
    RParentheses,
    LSquare,
    RSquare,
    LCurly,
    RCurly,
    LAngled,
    RAngled,
}

impl Brackets {
    fn opposite(&self) -> Brackets {
        match self {
            Self::LAngled => Self::RAngled,
            Self::RAngled => Self::LAngled,
            Self::LCurly => Self::RCurly,
            Self::RCurly => Self::LCurly,
            Self::LSquare => Self::RSquare,
            Self::RSquare => Self::LSquare,
            Self::LParentheses => Self::RParentheses,
            Self::RParentheses => Self::LParentheses,
        }
    }
    fn is_left(&self) -> bool {
        matches!(
            self,
            Self::LAngled | Self::LCurly | Self::LParentheses | Self::LSquare
        )
    }
    fn is_opposite(&self, rhs: Brackets) -> bool {
        self.opposite() == rhs
    }
    #[allow(clippy::match_like_matches_macro)]
    fn is_possible(&self, rhs: Brackets) -> bool {
        rhs.is_left() || rhs.opposite() == *self
    }
}

impl From<u8> for Brackets {
    fn from(value: u8) -> Self {
        match value {
            b'(' => Brackets::LParentheses,
            b')' => Brackets::RParentheses,
            b'[' => Brackets::LSquare,
            b']' => Brackets::RSquare,
            b'{' => Brackets::LCurly,
            b'}' => Brackets::RCurly,
            b'<' => Brackets::LAngled,
            b'>' => Brackets::RAngled,
            _ => panic!("Not a bracket - Brackets"),
        }
    }
}

pub fn task_10() {
    let file = read_file_bytes("./data_files/file10.txt");
    let mut stack: Vec<Brackets> = vec![];
    let mut scores_part1: Vec<BracketScores> = vec![];
    let mut scores_part2: Vec<i64> = vec![];
    for line in file.split(|&c| c as char == '\n') {
        stack.clear();
        for &c in line {
            let bracket: Brackets = c.into();
            if stack.is_empty() {
                stack.push(bracket);
                continue;
            }
            let last = *stack.last().expect("Stack was empty");
            match (last.is_possible(bracket), last.is_opposite(bracket)) {
                (true, false) => stack.push(bracket),
                (true, true) => {
                    stack.pop();
                }
                (false, _) => {
                    scores_part1.push(bracket.into());
                    stack.clear();
                    break;
                }
            }
        }
        if !stack.is_empty() {
            let score2 = stack.iter().rfold(0, |acc, &l_bracket| {
                acc * 5 + BracketScores::from(l_bracket).value2()
            });
            scores_part2.push(score2);
        }
    }
    let part1 = scores_part1
        .iter()
        .fold(0, |acc, score| acc + score.value1());
    dbg!(part1);
    scores_part2.sort();
    let part2 = scores_part2[scores_part2.len() / 2];
    dbg!(part2);
}
