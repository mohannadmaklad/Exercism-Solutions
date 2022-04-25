pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    user_ops: Vec<String>,
    user_code: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            user_code: vec![],
            user_ops: vec![],
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let l_in = input.to_lowercase();
        let input_exprs = get_user_expression(&l_in);
        let mut ret: Result = Ok(());

        for i in input_exprs {
            if i.chars().nth(0).unwrap() == ':' {
                ret = self.handle_declaration(&i);
            } else {
                let operation = i.split(' ').filter(|&s| s != ";");
                for op in operation {
                    //User Operation?
                    if self.user_ops.contains(&op.to_string()) {
                        ret = self.eval_user_operation(&op);
                        //Native Operation?
                    } else if self.is_operation(&op.to_string()) {
                        ret = self.eval_native_operation(&op.to_string());
                    } else {
                        //Is a number?
                        if op.parse::<Value>().is_ok() {
                            self.stack.push(op.parse::<Value>().unwrap());
                        } else {
                            return Result::Err(Error::UnknownWord);
                        }
                    }
                }
            }

            if ret.is_err() {
                return ret;
            }
        }
        Ok(())
    }

    pub fn eval_user_operation(&mut self, op: &str) -> Result {
        return self.eval(&self.operation_to_code(&op));
    }

    pub fn is_operation(&self, c: &str) -> bool {
        c == "+"
            || c == "-"
            || c == "*"
            || c == "/"
            || c == "dup"
            || c == "drop"
            || c == "swap"
            || c == "over"
            || c == ":"
    }
    pub fn eval_native_operation(&mut self, input: &str) -> Result {
        match input.as_ref() {
            "+" => {
                if self.stack.iter().any(|c| self.is_operation(&c.to_string()))
                    || self.stack.len() < 2
                {
                    return Result::Err(Error::StackUnderflow);
                }
                let sum: Value = self.stack.iter().sum();
                self.stack.clear();
                self.stack.push(sum);
            }

            "-" => {
                if self.stack.iter().any(|c| self.is_operation(&c.to_string()))
                    || self.stack.len() != 2
                {
                    return Result::Err(Error::StackUnderflow);
                }
                let small = self.stack.pop().unwrap();
                let big = self.stack.pop().unwrap();
                self.stack.push(big - small);
            }

            "*" => {
                if self.stack.iter().any(|c| self.is_operation(&c.to_string()))
                    || self.stack.len() != 2
                {
                    return Result::Err(Error::StackUnderflow);
                }
                let small = self.stack.pop().unwrap();
                let big = self.stack.pop().unwrap();
                self.stack.push(big * small);
            }
            "/" => {
                if self.stack.iter().any(|c| self.is_operation(&c.to_string()))
                    || self.stack.len() != 2
                {
                    return Result::Err(Error::StackUnderflow);
                }
                let small = self.stack.pop().unwrap();
                let big = self.stack.pop().unwrap();

                if small == 0 {
                    return Result::Err(Error::DivisionByZero);
                }
                self.stack.push(big / small);
            }
            "dup" => {
                if self.stack.is_empty() {
                    return Result::Err(Error::StackUnderflow);
                }
                let last = self.stack.pop().unwrap();
                self.stack.push(last);
                self.stack.push(last);
            }
            "drop" => {
                if self.stack.is_empty() {
                    return Result::Err(Error::StackUnderflow);
                }
                self.stack.pop();
            }
            "swap" => {
                if self.stack.len() < 2 {
                    return Result::Err(Error::StackUnderflow);
                }
                let last = self.stack.pop().unwrap();
                let before_last = self.stack.pop().unwrap();

                self.stack.push(last);
                self.stack.push(before_last);
            }
            "over" => {
                if self.stack.len() < 2 {
                    return Result::Err(Error::StackUnderflow);
                }
                let last = self.stack.pop().unwrap();
                let before_last = self.stack.pop().unwrap();

                self.stack.push(before_last);
                self.stack.push(last);
                self.stack.push(before_last);
            }

            _ => {}
        }

        return Ok(());
    }

    fn handle_declaration(&mut self, input: &str) -> Result {
        //Check for valid declaration
        if input.len() < 3
            || input.chars().nth(0).unwrap() != ':'
            || input.chars().nth(input.len() - 1).unwrap() != ';'
        {
            return Err(Error::InvalidWord);
        }

        let operation: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
        let op = operation[1].clone();

        //Check if the operation is a number
        if matches!(op.parse::<i32>(), Ok(_)) {
            return Result::Err(Error::InvalidWord);
        }

        let new_user_code = operation
            .iter()
            .skip(2)
            .map(|o| self.operation_to_code(&o) + " ")
            .collect::<String>()
            .trim()
            .to_string();

        if new_user_code.len() > 20 {
            return Ok(());
        }
        //Check if the operation is already defined
        let op_idx = self.user_ops.iter().position(|v| *v == op);

        match op_idx {
            Some(i) => {
                self.user_code[i] = new_user_code;
            }
            None => {
                self.user_ops.push(op);
                self.user_code.push(new_user_code);
            }
        }
        Ok(())
    }

    fn operation_to_code(&self, operation: &str) -> String {
        let op_idx = self.user_ops.iter().position(|v| *v == operation);
        match op_idx {
            Some(i) => self.user_code[i].clone(),
            _ => operation.to_string(),
        }
    }
}

//Expected to return expressions in the correct order
//Example input : "1 2 + : addone 1 + ; addone"
//Expected output: [1 2 +] [: addone 1 + ;] [addone]
fn get_user_expression(input: &str) -> Vec<String> {
    let mut ops: Vec<String> = vec![];
    let mut op: Vec<char> = vec![];

    for (i, c) in input.chars().enumerate() {
        if c == ':' && !op.iter().collect::<String>().trim().is_empty() {
            ops.push(op.iter().collect::<String>().trim().to_string());
            op.clear();
            op.push(c);
        } else if c == ';' && !op.iter().collect::<String>().trim().is_empty() {
            op.push(c);
            ops.push(op.iter().collect::<String>().trim().to_string());
            op.clear();
        } else {
            op.push(c)
        }
        if i == input.len() - 1 && !op.iter().collect::<String>().trim().is_empty() {
            ops.push(op.iter().collect::<String>().trim().to_string());
        }
    }
    ops
}
