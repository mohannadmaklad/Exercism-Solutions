#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut res = vec![0; 0];
    for input in inputs {
        match input {
            CalculatorInput::Value(val) => res.push(*val),
            _ => match res.len() {
                2.. => do_operation(&mut res, input),
                _ => return None,
            },
        }
    }

    match res.len() {
        1 => Some(res[0]),
        _ => None,
    }
}

fn do_operation(input: &mut Vec<i32>, operation: &CalculatorInput) {
    match input.len() {
        2.. => {
            let left = input.pop().unwrap();
            let right = input.pop().unwrap();

            match operation {
                CalculatorInput::Add => input.push(right + left),
                CalculatorInput::Divide => input.push(right / left),
                CalculatorInput::Multiply => input.push(right * left),
                CalculatorInput::Subtract => input.push(right - left),
                _ => (),
            }
        }
        _ => (),
    }
}
