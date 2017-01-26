struct Instruction {
    op: Operation,
    dots: usize,
    hearts: HeartTree,
}

struct Operation {
    op_type: OperationType,
    hangul_count: usize,
}

enum OperationType {
    Push,           // 형
    Add,            // 항
    Multiply,       // 핫
    Negate,         // 흣
    Reciprocate,    // 흡
    Duplicate,      // 흑
}

enum HeartTree {
    Heart(usize),
    Return,
    LessThan(Box<HeartTree>, Box<HeartTree>),
    Equals(Box<HeartTree>, Box<HeartTree>),
}

fn main() {
}
