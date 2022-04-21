pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let closing_bracket = |c: char| match c {
        '}' => '{',
        ')' => '(',
        ']' => '[',
        _ => '!',
    };

    for c in string.chars() {
        if c == '{' || c == '(' || c == '[' {
            stack.push(c);
        } else if (c == '}' || c == ')' || c == ']') && stack.pop() != Some(closing_bracket(c)) {
            return false;
        }
    }

    stack.is_empty()
}
