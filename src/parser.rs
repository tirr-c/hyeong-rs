use super::structure::{Operation, Instruction, HeartTree};
use std::str;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl Into<char> for HangulStartType {
    fn into(self) -> char {
        match self {
            HangulStartType::Hyeo   => '혀',
            HangulStartType::Ha     => '하',
            HangulStartType::Heu    => '흐',
            HangulStartType::Hyeong => '형',
            HangulStartType::Hang   => '항',
            HangulStartType::Hat    => '핫',
            HangulStartType::Heut   => '흣',
            HangulStartType::Heup   => '흡',
            HangulStartType::Heuk   => '흑',
        }
    }
}

#[cfg(test)]
#[test]
fn hangul_start_type_from_char() {
    assert_eq!(HangulStartType::from_char('혀'), Some(HangulStartType::Hyeo));
    assert_eq!(HangulStartType::from_char('하'), Some(HangulStartType::Ha));
    assert_eq!(HangulStartType::from_char('흐'), Some(HangulStartType::Heu));
    assert_eq!(HangulStartType::from_char('형'), Some(HangulStartType::Hyeong));
    assert_eq!(HangulStartType::from_char('항'), Some(HangulStartType::Hang));
    assert_eq!(HangulStartType::from_char('핫'), Some(HangulStartType::Hat));
    assert_eq!(HangulStartType::from_char('흣'), Some(HangulStartType::Heut));
    assert_eq!(HangulStartType::from_char('흡'), Some(HangulStartType::Heup));
    assert_eq!(HangulStartType::from_char('흑'), Some(HangulStartType::Heuk));
    assert_eq!(HangulStartType::from_char('엉'), None);
    assert_eq!(HangulStartType::from_char('앙'), None);
    assert_eq!(HangulStartType::from_char('앗'), None);
    assert_eq!(HangulStartType::from_char('.'), None);
    assert_eq!(HangulStartType::from_char('?'), None);
    assert_eq!(HangulStartType::from_char('♥'), None);
}

#[cfg(test)]
#[test]
fn hangul_start_type_self_ending() {
    assert!(!  HangulStartType::Hyeo.is_self_ending());
    assert!(!    HangulStartType::Ha.is_self_ending());
    assert!(!   HangulStartType::Heu.is_self_ending());
    assert!( HangulStartType::Hyeong.is_self_ending());
    assert!(   HangulStartType::Hang.is_self_ending());
    assert!(    HangulStartType::Hat.is_self_ending());
    assert!(   HangulStartType::Heut.is_self_ending());
    assert!(   HangulStartType::Heup.is_self_ending());
    assert!(   HangulStartType::Heuk.is_self_ending());
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
    Dot,
    ThreeDots,
    Heart(usize),
    ReturnHeart,
    ExclamationMark,
    QuestionMark,
}

const HEART_MARKS: [char; 11] = [
    '\u{2665}', '\u{2764}', '\u{1f495}', '\u{1f496}', '\u{1f497}', '\u{1f498}',
    '\u{1f499}', '\u{1f49a}', '\u{1f49b}', '\u{1f49c}', '\u{1f49d}'
];

impl Token {
    pub fn from_char(c: char) -> Option<Self> {
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

#[cfg(test)]
#[test]
fn token_from_char() {
    assert_eq!(Token::from_char('.'), Some(Token::Dot));
    assert_eq!(Token::from_char('!'), Some(Token::ExclamationMark));
    assert_eq!(Token::from_char('?'), Some(Token::QuestionMark));
    assert_eq!(Token::from_char('\u{2026}'), Some(Token::ThreeDots));
    assert_eq!(Token::from_char('\u{22ee}'), Some(Token::ThreeDots));
    assert_eq!(Token::from_char('\u{22ef}'), Some(Token::ThreeDots));
    assert_eq!(Token::from_char('\u{2661}'), Some(Token::ReturnHeart));
}

#[cfg(test)]
#[test]
fn token_from_char_hearts() {
    // marker heart symbol used in hyeong-lang
    for (i, c) in HEART_MARKS.iter().enumerate() {
        assert_eq!(Token::from_char(*c), Some(Token::Heart(i)));
    }
    // white heart suit
    assert_eq!(Token::from_char('\u{2661}'), Some(Token::ReturnHeart));

    // some random hearts
    assert_eq!(Token::from_char('\u{2765}'), None);
    assert_eq!(Token::from_char('\u{1f49e}'), None);
}


pub struct Parser<'a> {
    code: str::Chars<'a>,
    operation_cache: Option<Operation>,
    token_cache: VecDeque<Token>,
}

impl<'a> Parser<'a> {
    pub fn from_str(code: &'a str) -> Self {
        let mut parser = Parser {
            code: code.chars(),
            operation_cache: None,
            token_cache: VecDeque::new(),
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
                return Some(Operation::from_chars(start.into(), None, 1));
            }
            let mut temp_iter = self.code.clone();
            if let Some((count, c)) = Parser::find_matching_end(start, &mut temp_iter) {
                self.code = temp_iter;
                let length = count + 1;
                return Some(Operation::from_chars(start.into(), Some(c), length));
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
        let mut current_heart = None;
        let mut tree = vec![];
        let mut op_count = 0;
        let hearts = self.token_cache.iter().filter(|token| {
            match **token {
                Token::Dot | Token::ThreeDots => false,
                _ => true,
            }
        });
        for token in hearts {
            match *token {
                Token::Heart(id) => {
                    current_heart = current_heart.or(Some(HeartTree::Heart(id)));
                },
                Token::ReturnHeart => {
                    current_heart = current_heart.or(Some(HeartTree::Return));
                },
                Token::ExclamationMark => {
                    tree.push(current_heart.unwrap_or(HeartTree::Nil));
                    current_heart = None;
                    op_count += 1;
                },
                Token::QuestionMark => {
                    tree.push(current_heart.unwrap_or(HeartTree::Nil));
                    current_heart = None;
                    for _ in 0..op_count {
                        let rhs = tree.pop().unwrap();
                        let lhs = tree.pop().unwrap();
                        tree.push(HeartTree::Equals(Box::new(lhs), Box::new(rhs)));
                    }
                    op_count = 0;
                },
                _ => { },
            }
        }
        tree.push(current_heart.unwrap_or(HeartTree::Nil));
        for _ in 0..op_count {
            let rhs = tree.pop().unwrap();
            let lhs = tree.pop().unwrap();
            tree.push(HeartTree::Equals(Box::new(lhs), Box::new(rhs)));
        }
        while tree.len() > 1 {
            let rhs = tree.pop().unwrap();
            let lhs = tree.pop().unwrap();
            tree.push(HeartTree::LessThan(Box::new(lhs), Box::new(rhs)));
        }
        Some(Instruction::new(op, dots, tree.pop().unwrap_or(HeartTree::Nil)))
    }
}

#[cfg(test)]
#[test]
fn parser_simple() {
    let mut parser = Parser::from_str("혀엉...");
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('혀', Some('엉'), 2), 3, HeartTree::Nil))
        );
    assert_eq!(parser.next(), None);
}

#[cfg(test)]
#[test]
fn parser_self_ending() {
    let mut parser = Parser::from_str("형 항. 핫... 흡.. 흑. 흣.....");
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('형', None, 1), 0, HeartTree::Nil))
        );
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('항', None, 1), 1, HeartTree::Nil))
        );
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('핫', None, 1), 3, HeartTree::Nil))
        );
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('흡', None, 1), 2, HeartTree::Nil))
        );
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('흑', None, 1), 1, HeartTree::Nil))
        );
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('흣', None, 1), 5, HeartTree::Nil))
        );
    assert_eq!(parser.next(), None);
}

#[cfg(test)]
#[test]
fn parser_noop() {
    let mut parser = Parser::from_str("흐으응... 너무 커엇...");
    assert_eq!(parser.next(), None);
}

#[cfg(test)]
#[test]
fn parser_multiple() {
    let mut parser = Parser::from_str("혀엉... 흑. 흐읏..... 하아아앙...");
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('혀', Some('엉'), 2), 3, HeartTree::Nil))
        );
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('흑', None, 1), 1, HeartTree::Nil))
        );
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('흐', Some('읏'), 2), 5, HeartTree::Nil))
        );
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('하', Some('앙'), 4), 3, HeartTree::Nil))
        );
    assert_eq!(parser.next(), None);
}

#[cfg(test)]
#[test]
fn parser_hangul_syllables() {
    // WHAT AM I DOING
    let mut parser = Parser::from_str("혀내 이름은 메구밍!엉... 흐아크 위저드를 생업으로 삼고 있으며읍..... 최강의 공격마법, 하폭렬마법앙....을 흐으으... 펼치는 자아읏...!");
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('혀', Some('엉'), 9), 3, HeartTree::Nil))
        );
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('흐', Some('읍'), 17), 5, HeartTree::Nil))
        );
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('하', Some('앙'), 6), 4, HeartTree::Nil))
        );
    assert_eq!(
        parser.next(),
        Some(Instruction::new(Operation::from_chars('흐', Some('읏'), 9), 3,
             HeartTree::Equals(Box::new(HeartTree::Nil), Box::new(HeartTree::Nil))))
        );
    assert_eq!(parser.next(), None);
}
