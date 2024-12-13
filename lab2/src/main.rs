use std::io;
use std::collections::VecDeque;
use std::str::FromStr;

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn apply_operator(op: char, b: f64, a: f64) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b == 0.0 {
                panic!("Помилка! Ділення на нуль!");
            }
            a / b
        }
        _ => 0.0,
    }
}

fn tokenize(expression: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut number = String::new();

    for ch in expression.chars() {
        if ch.is_digit(10) || ch == '.' {
            number.push(ch);
        } else {
            if !number.is_empty() {
                tokens.push(number.clone());
                number.clear();
            }

            if !ch.is_whitespace() {
                tokens.push(ch.to_string());
            }
        }
    }

    return tokens
}

fn evaluate_expression(expression: &str) -> f64 {
    let mut operators: VecDeque<char> = VecDeque::new();
    let mut operands: VecDeque<f64> = VecDeque::new();
    
    let tokens = tokenize(expression);

    for token in tokens {
        if let Ok(num) = f64::from_str(&token) {
            operands.push_back(num);
        } else if token == "(" {
            operators.push_back('(');
        } else if token == ")" {
            while let Some(op) = operators.pop_back() {
                if op == '(' {
                    break;
                }
                let b = operands.pop_back().unwrap();
                let a = operands.pop_back().unwrap();
                operands.push_back(apply_operator(op, b, a));
            }
        } else {
            let current_op = token.chars().next().unwrap();
            while let Some(op) = operators.pop_back() {
                if precedence(op) < precedence(current_op) {
                    operators.push_back(op);
                    break;
                }
                let b = operands.pop_back().unwrap();
                let a = operands.pop_back().unwrap();
                operands.push_back(apply_operator(op, b, a));
            }
            operators.push_back(current_op);
        }
    }
    
    while let Some(op) = operators.pop_back() {
        let b = operands.pop_back().unwrap();
        let a = operands.pop_back().unwrap();
        operands.push_back(apply_operator(op, b, a));
    }

    let result = operands.pop_back().unwrap();
    result
}

fn main() {
    let mut input = String::new();
    println!("Введіть вираз: ");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Не вдалося прочитати рядок");
    
    let result = evaluate_expression(&input);
    println!("Результат: {:.3}", result);
}
