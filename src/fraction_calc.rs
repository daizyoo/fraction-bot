use std::fmt::Debug;

fn _main() {
    let fraction = fraction_calculation("1/3 + -2/6 × 3/5");
    println!("{:?}", fraction)
}

pub fn fraction_calculation(formula: &str) -> Fraction {
    let mut formula: Formula = to_formula(formula);

    print_formula(&formula);

    while let Some(i) = formula.iter().position(|item| match item {
        Item::Fraction(_) => false,
        Item::Symbol(s) => match s {
            Symbol::Times | Symbol::Divided => true,
            _ => false,
        },
    }) {
        let fraction = calculation(
            formula[i].symbol(),
            formula[i - 1].fraction(),
            formula[i + 1].fraction(),
        );

        for _ in 0..2 {
            formula.remove(i - 1);
        }
        formula[i - 1] = Item::Fraction(fraction);
        print_formula(&formula);
    }

    while formula.len() != 1 {
        let fraction = calculation(
            formula[1].symbol(),
            formula[0].fraction(),
            formula[2].fraction(),
        );

        for _ in 0..2 {
            formula.remove(0);
        }
        formula[0] = Item::Fraction(fraction);

        print_formula(&formula);
    }

    let result = formula[0].fraction();
    let numerator = result.numerator;
    let denominator = result.denominator;

    let divide = reduction(numerator, denominator);

    Fraction::new(numerator / divide, denominator / divide, result.sign)
}

/// 計算
fn calculation(mut symbol: Symbol, mut f1: Fraction, mut f2: Fraction) -> Fraction {
    let mut sign = true;
    if symbol == Symbol::Minus || symbol == Symbol::Plus {
        if !f1.sign {
            let f = f1;
            f1 = f2;
            f2 = f;
            symbol = Symbol::Minus;
        } else if !f2.sign {
            f2.sign = true;
            let f = f1;
            f1 = f2;
            f2 = f;
            symbol = Symbol::Minus;
        } else if !f1.sign && !f2.sign {
            sign = false;
            symbol = Symbol::Plus;
        }
    }

    match symbol {
        Symbol::Plus => {
            let (f1_m, f2_m, d) = common_multiple(f1.denominator, f2.denominator);
            Fraction::new((f1.numerator * f1_m) + (f2.numerator * f2_m), d, sign)
        }
        Symbol::Minus => {
            let (f1_m, f2_m, d) = common_multiple(f1.denominator, f2.denominator);
            Fraction::new((f1.numerator * f1_m) - (f2.numerator * f2_m), d, sign)
        }
        Symbol::Times => {
            if !f1.sign || !f2.sign {
                Fraction::new(
                    f1.numerator * f2.numerator,
                    f1.denominator * f2.denominator,
                    false,
                )
            } else {
                Fraction::new(
                    f1.numerator * f2.numerator,
                    f1.denominator * f2.denominator,
                    true,
                )
            }
        }
        Symbol::Divided => {
            if !f1.sign || !f2.sign {
                Fraction::new(
                    f1.numerator * f2.numerator,
                    f1.denominator * f2.denominator,
                    false,
                )
            } else {
                Fraction::new(
                    f1.numerator * f2.numerator,
                    f1.denominator * f2.denominator,
                    true,
                )
            }
        }
    }
}

// 公倍数
fn common_multiple(num1: i64, num2: i64) -> (i64, i64, i64) {
    let mut v1 = vec![num1];
    let mut v2 = vec![num2];

    let mut n = 2;

    loop {
        if v1.last().unwrap() < v2.last().unwrap() {
            v1.push(num1 * n);
        }
        if v2.last().unwrap() < v1.last().unwrap() {
            v2.push(num2 * n);
        }

        for (i, num) in v1.iter().enumerate() {
            if let Some(i2) = v2.iter().position(|n| n == num) {
                return (i as i64 + 1, i2 as i64 + 1, *num);
            }
        }

        n += 1;
    }
}

/// 公約数
fn reduction(num1: i64, num2: i64) -> i64 {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for i in 1..=num1 {
        if num1 % i == 0 {
            v1.push(i);
        }
    }
    for i in 1..=num2 {
        if num2 % i == 0 {
            v2.push(i);
        }
    }

    for num in v1.iter().rev() {
        if let Some(result) = v2.iter().find(|&x| x == num) {
            return *result;
        }
    }

    1
}

/// 文字列を式に変換
fn to_formula(str: &str) -> Formula {
    let formula = str.split(' ').map(|str| str.into()).collect();
    if !check(&formula) {
        panic!()
    }
    formula
}

/// 式が成り立つか判定
fn check(formula: &Formula) -> bool {
    if formula.len() % 2 == 0 {
        return false;
    }

    for (i, item) in formula.iter().enumerate() {
        match *item {
            Item::Fraction(_) => {
                if i % 2 != 0 {
                    return false;
                }
            }
            Item::Symbol(_) => {
                if i % 2 == 0 {
                    return false;
                }
            }
        }
    }
    true
}

fn print_formula(formula: &Formula) {
    formula.iter().for_each(|item| print!("{:?} ", item));
    println!()
}

/// 式
type Formula = Vec<Item>;

enum Item {
    Symbol(Symbol),
    Fraction(Fraction),
}

/// 記号
#[derive(Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Plus,
    Minus,
    Times,
    Divided,
}

/// 分数
#[derive(Clone, Copy)]
pub struct Fraction {
    /// 分子
    numerator: i64,
    /// 分母
    denominator: i64,
    /// 符号
    sign: bool,
}

/// 式をItemに変換
const fn fraction(numerator: i64, denominator: i64, sign: bool) -> Item {
    Item::Fraction(Fraction::new(numerator, denominator, sign))
}

const PLUS: Item = Item::Symbol(Symbol::Plus);
const MINUS: Item = Item::Symbol(Symbol::Minus);
const TIMES: Item = Item::Symbol(Symbol::Times);
const DIVIDED: Item = Item::Symbol(Symbol::Divided);

impl Item {
    const fn fraction(&self) -> Fraction {
        match self {
            Item::Fraction(f) => *f,
            Item::Symbol(_) => panic!(),
        }
    }
    const fn symbol(&self) -> Symbol {
        match self {
            Item::Fraction(_) => panic!(),
            Item::Symbol(s) => *s,
        }
    }
}

impl Fraction {
    const fn new(numerator: i64, denominator: i64, sign: bool) -> Fraction {
        Fraction {
            numerator,
            denominator,
            sign,
        }
    }
}

impl ToString for Fraction {
    fn to_string(&self) -> String {
        format!("{}{}/{}", self.sign, self.numerator, self.denominator)
    }
}

impl From<&str> for Item {
    fn from(str: &str) -> Self {
        match str {
            "+" => PLUS,
            "-" => MINUS,
            "×" => TIMES,
            "÷" => DIVIDED,
            _ => {
                let nums: Vec<i64> = str.split('/').map(|s| s.trim().parse().unwrap()).collect();
                let mut sign = true;
                let mut n = nums[0];
                let mut d = nums[1];
                if n < 0 && d < 0 {
                    n *= -1;
                    d *= -1;
                    sign = true;
                } else if n < 0 {
                    n *= -1;
                    sign = false;
                } else if d < 0 {
                    d *= -1;
                    sign = false;
                }
                fraction(n as i64, d as i64, sign)
            }
        }
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}/{}",
            if self.sign { "" } else { "-" },
            self.numerator,
            self.denominator
        )
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Symbol::Plus => "+",
                Symbol::Minus => "-",
                Symbol::Times => "×",
                Symbol::Divided => "÷",
            }
        )
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Fraction(fraction) => write!(f, "{:?}", fraction),
            Item::Symbol(symbol) => write!(f, "{:?}", symbol),
        }
    }
}
