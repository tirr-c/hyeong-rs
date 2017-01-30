#[derive(Debug)]
struct Instruction {
    op: Operation,
    dots: usize,
    hearts: HeartTree,
}

#[derive(Copy, Clone, Debug)]
struct Operation {
    op_type: OperationType,
    hangul_count: usize,
}

impl Operation {
    fn from_chars(start: HangulStartType, end: Option<char>, count: usize) -> Self {
        assert!(end.is_none() == start.is_self_ending());
        if let Some(c) = end {
            Operation { op_type: match c {
                '엉' => OperationType::Push,
                '앙' => OperationType::Add,
                '앗' => OperationType::Multiply,
                '읏' => OperationType::Negate,
                '읍' => OperationType::Reciprocate,
                '윽' => OperationType::Duplicate,
                _ => panic!("Invalid end character")
            }, hangul_count: count }
        } else {
            Operation { op_type: match start {
                HangulStartType::Hyeong => OperationType::Push,
                HangulStartType::Hang   => OperationType::Add,
                HangulStartType::Hat    => OperationType::Multiply,
                HangulStartType::Heut   => OperationType::Negate,
                HangulStartType::Heup   => OperationType::Reciprocate,
                HangulStartType::Heuk   => OperationType::Duplicate,
                _ => panic!("Should not happen")
            }, hangul_count: 1 }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum OperationType {
    Push,           // 형
    Add,            // 항
    Multiply,       // 핫
    Negate,         // 흣
    Reciprocate,    // 흡
    Duplicate,      // 흑
}

#[derive(Debug)]
enum HeartTree {
    Heart(usize),
    Return,
    LessThan(Box<HeartTree>, Box<HeartTree>),
    Equals(Box<HeartTree>, Box<HeartTree>),
    Nil,
}

#[derive(Copy, Clone, Debug)]
enum HangulStartType {
    Hyeo,
    Ha,
    Heu,
    Hyeong,
    Hang,
    Hat,
    Heut,
    Heup,
    Heuk,
}

impl HangulStartType {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '혀' => Some(HangulStartType::Hyeo  ),
            '하' => Some(HangulStartType::Ha    ),
            '흐' => Some(HangulStartType::Heu   ),
            '형' => Some(HangulStartType::Hyeong),
            '항' => Some(HangulStartType::Hang  ),
            '핫' => Some(HangulStartType::Hat   ),
            '흣' => Some(HangulStartType::Heut  ),
            '흡' => Some(HangulStartType::Heup  ),
            '흑' => Some(HangulStartType::Heuk  ),
            _    => None,
        }
    }

    fn is_self_ending(&self) -> bool {
        match *self {
            HangulStartType::Hyeo => false,
            HangulStartType::Ha   => false,
            HangulStartType::Heu  => false,
            _ => true,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Token {
    Dot,
    ThreeDots,
    Heart(usize),
    ReturnHeart,
    ExclamationMark,
    QuestionMark,
}

const HEART_MARKS: [char; 11] = [
    '♥', '❤', '💕', '💖', '💗', '💘', '💙', '💚', '💛', '💜', '💝'
];

impl Token {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Token::Dot),
            '\u{2026}' | '\u{22ee}' | '\u{22ef}' => Some(Token::ThreeDots),
            '\u{2661}' => Some(Token::ReturnHeart),
            '!' => Some(Token::ExclamationMark),
            '?' => Some(Token::QuestionMark),
            _ => HEART_MARKS.iter().position(|&i| i == c).map(|p| Token::Heart(p))
        }
    }
}

struct Parser<'a> {
    code: std::str::Chars<'a>,
    operation_cache: Option<Operation>,
    token_cache: std::collections::VecDeque<Token>,
}

impl<'a> Parser<'a> {
    fn from_str(code: &'a str) -> Self {
        let mut parser = Parser {
            code: code.chars(),
            operation_cache: None,
            token_cache: std::collections::VecDeque::new(),
        };
        // First run
        let hangul = parser.parse_hangul();
        parser.operation_cache = hangul;
        parser
    }

    fn parse_hangul(&mut self) -> Option<Operation> {
        self.token_cache.clear();
        loop {
            let mut start = None;
            while let Some(c) = self.code.next() {
                if "형항핫흣흡흑혀하흐".contains(c) {
                    start = HangulStartType::from_char(c);
                    break;
                }
                if let Some(token) = Token::from_char(c) {
                    self.token_cache.push_back(token);
                }
            }
            let start = match start {
                Some(item) => item,
                None => { return None; }
            };
            if start.is_self_ending() {
                return Some(Operation::from_chars(start, None, 1));
            }
            let mut temp_iter = self.code.clone();
            if let Some((count, c)) = Parser::find_matching_end(start, &mut temp_iter) {
                self.code = temp_iter;
                let length = count + 1;
                return Some(Operation::from_chars(start, Some(c), length));
            }
        }
    }

    fn find_matching_end<T: Iterator<Item=char>>(
        start: HangulStartType, iter: &mut T
        ) -> Option<(usize, char)> {
        let mut cnt = 0;
        while let Some(c) = iter.next() {
            if c >= '가' && c <= '힣' { cnt += 1; }
            let end = match start {
                HangulStartType::Hyeo => '엉' == c,
                HangulStartType::Ha   => ['앙', '앗'].contains(&c),
                HangulStartType::Heu  => ['읏', '읍', '윽'].contains(&c),
                _ => false
            };
            if end { return Some((cnt, c)); }
        }
        None
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Instruction;
    fn next(&mut self) -> Option<Self::Item> {
        let op = match self.operation_cache {
            Some(op) => op,
            None => { return None; },
        };
        let next_op = self.parse_hangul();
        self.operation_cache = next_op;

        // dots
        let tokens = self.token_cache.iter().take_while(|token| {
            match **token {
                Token::Dot | Token::ThreeDots => true,
                _ => false,
            }
        });
        let dots = tokens.fold(0, |i, token| {
            match *token {
                Token::Dot => i + 1,
                Token::ThreeDots => i + 3,
                _ => i,
            }
        });
        // hearts
        let hearts = self.token_cache.iter().filter(|token| {
            match **token {
                Token::Dot | Token::ThreeDots => false,
                _ => true,
            }
        });
        // TODO: parse hearts
        Some(Instruction { op: op, dots: dots, hearts: HeartTree::Nil })
    }
}

fn main() {
    let parser = Parser::from_str("하흐아읏...하아앙....흑..♥.혀엉...");
    for op in parser {
        println!("{:?}", op);
    }
}
