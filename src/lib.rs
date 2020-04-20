use std::vec::Vec;
use std::collections::LinkedList;

fn is_operand(token: &String) -> bool {
    return token.parse::<i64>().is_ok();
}

fn is_operator(token: &String) -> bool {
    return !is_operand(token);
}

pub fn shunting_yard(token_list: Vec<String>) -> Result<Vec<String>, String> {
    if token_list.is_empty() {
        return Err("Empty token list".to_owned());
    }

    let mut output_queue: Vec<String> = Vec::new();
    let mut operator_stack: LinkedList<String> = LinkedList::new();
    for i in 0..token_list.len() {
        let token = token_list[i].clone();
        if is_operator(&token) {
            while !operator_stack.is_empty() {
                let popped_operator: String = operator_stack.pop_back().unwrap();
                output_queue.push(popped_operator);
            }
            operator_stack.push_back(token);
        } else {
            output_queue.push(token);
        }
    }
    while !operator_stack.is_empty() {
        let popped_operator: String = operator_stack.pop_back().unwrap();
        output_queue.push(popped_operator);
    }
    return Ok(output_queue);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_expression() {
        let token = Vec::new();
        let result = shunting_yard(token);
        assert_eq!(false, result.is_ok());
    }

    #[test]
    fn simple_expression() {
        let token: Vec<String> = "1 + 1".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let result = shunting_yard(token);
        assert_eq!(true, result.is_ok());

        let expected_token: Vec<String> = "1 1 +".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(expected_token, result.unwrap());
    }

    #[test]
    fn simple_expression_two_operators() {
        let token: Vec<String> = "1 + 2 - 3".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let result = shunting_yard(token);
        assert_eq!(true, result.is_ok());

        let expected_token: Vec<String> = "1 2 + 3 -".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(expected_token, result.unwrap());
    }
}
