#[derive(Debug, PartialEq)]
pub struct Instruction {
    op: Operation,
    dots: usize,
    hearts: HeartTree,
}

impl Instruction {
    pub fn new(op: Operation, dots: usize, hearts: HeartTree) -> Self {
        Instruction {
            op: op,
            dots: dots,
            hearts: hearts,
        }
    }

    pub fn operation_type(&self) -> OperationType {
        self.op.op_type
    }

    pub fn hangul_count(&self) -> usize {
        self.op.hangul_count
    }

    pub fn dots(&self) -> usize {
        self.dots
    }

    pub fn hangul_times_dots(&self) -> usize {
        self.op.hangul_count * self.dots
    }

    pub fn heart_tree(&self) -> &HeartTree {
        &self.hearts
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Operation {
    op_type: OperationType,
    hangul_count: usize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OperationType {
    Push,        // 형
    Add,         // 항
    Multiply,    // 핫
    Negate,      // 흣
    Reciprocate, // 흡
    Duplicate,   // 흑
}

impl Operation {
    pub fn from_chars(start: char, end: Option<char>, count: usize) -> Self {
        if let Some(c) = end {
            assert!(
                match start {
                    '혀' => '엉' == end.unwrap(),
                    '하' => "앙앗".contains(end.unwrap()),
                    '흐' => "읏읍윽".contains(end.unwrap()),
                    _ => false,
                },
                "Invalid start-end character pair"
            );
            Operation {
                op_type: match c {
                    '엉' => OperationType::Push,
                    '앙' => OperationType::Add,
                    '앗' => OperationType::Multiply,
                    '읏' => OperationType::Negate,
                    '읍' => OperationType::Reciprocate,
                    '윽' => OperationType::Duplicate,
                    _ => unreachable!(),
                },
                hangul_count: count,
            }
        } else {
            Operation {
                op_type: match start {
                    '형' => OperationType::Push,
                    '항' => OperationType::Add,
                    '핫' => OperationType::Multiply,
                    '흣' => OperationType::Negate,
                    '흡' => OperationType::Reciprocate,
                    '흑' => OperationType::Duplicate,
                    _ => panic!("Non-self-ending character without end character"),
                },
                hangul_count: 1,
            }
        }
    }

    #[cfg(test)]
    pub fn from_single_char(op: char, count: usize) -> Self {
        Operation {
            op_type: match op {
                '형' => OperationType::Push,
                '항' => OperationType::Add,
                '핫' => OperationType::Multiply,
                '흣' => OperationType::Negate,
                '흡' => OperationType::Reciprocate,
                '흑' => OperationType::Duplicate,
                _ => panic!("Invalid character"),
            },
            hangul_count: count,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HeartTree {
    Heart(usize),
    Return,
    LessThan(Box<HeartTree>, Box<HeartTree>),
    Equals(Box<HeartTree>, Box<HeartTree>),
    Nil,
}
