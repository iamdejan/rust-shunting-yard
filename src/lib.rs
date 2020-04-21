use std::vec::Vec;
use std::collections::LinkedList;

fn is_operand(token: &String) -> bool {
    return token.parse::<i64>().is_ok();
}

fn is_operator(token: &String) -> bool {
    return !is_operand(token) && token != "(" && token != ")";
}

fn get_operator_level(token: &String) -> i64 {
    if token == "*" || token == "/" {
        return 2;
    }

    return 1;
}

fn left_operator_has_greater_precedence(left_token: &String, right_token: &String) -> bool {
    return get_operator_level(left_token) >= get_operator_level(right_token);
}

pub fn shunting_yard(token_list: Vec<String>) -> Result<Vec<String>, String> {
    if token_list.is_empty() {
        return Err("Empty token list".to_owned());
    }

    let mut output_queue: Vec<String> = Vec::new();
    let mut operator_stack: LinkedList<String> = LinkedList::new();
    for i in 0..token_list.len() {
        let token = token_list[i].clone();
        if is_operand(&token) {
            output_queue.push(token);
        } else if is_operator(&token) {
            while !operator_stack.is_empty() && left_operator_has_greater_precedence(operator_stack.back().unwrap(), &token) {
                if operator_stack.back().unwrap() == "(" || operator_stack.back().unwrap() == ")" {
                    break;
                }
                let popped_operator: String = operator_stack.pop_back().unwrap();
                output_queue.push(popped_operator);
            }
            operator_stack.push_back(token);
        } else if token == "(" {
            operator_stack.push_back(token);
        } else if token == ")" {
            while !operator_stack.is_empty() && operator_stack.back().unwrap() != "(" {
                let popped_operator: String = operator_stack.pop_back().unwrap();
                output_queue.push(popped_operator);
            }
            operator_stack.pop_back().unwrap();
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
        let token_list: Vec<String> = "1 + 1".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let result = shunting_yard(token_list);
        assert_eq!(true, result.is_ok());

        let expected_token: Vec<String> = "1 1 +".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(expected_token, result.unwrap());
    }

    #[test]
    fn simple_expression_two_operators() {
        let token_list: Vec<String> = "1 + 2 - 3".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let result = shunting_yard(token_list);
        assert_eq!(true, result.is_ok());

        let expected_token: Vec<String> = "1 2 + 3 -".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(expected_token, result.unwrap());
    }

    #[test]
    fn simple_expression_two_operators_multiplication_and_division() {
        let token_list: Vec<String> = "1 * 2 / 1".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let result = shunting_yard(token_list);
        assert_eq!(true, result.is_ok());

        let expected_token: Vec<String> = "1 2 * 1 /".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(expected_token, result.unwrap());
    }

    #[test]
    fn expression_two_operators_two_precedences() {
        let token_list: Vec<String> = "1 + 2 * 3".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let result = shunting_yard(token_list);
        assert_eq!(true, result.is_ok());

        let expected_token: Vec<String> = "1 2 3 * +".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(expected_token, result.unwrap());
    }

    #[test]
    fn expression_with_parentheses() {
        let token_list: Vec<String> = "4 + 18 / ( 9 - 3 )".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let result = shunting_yard(token_list);
        assert_eq!(true, result.is_ok());

        let expected_token: Vec<String> = "4 18 9 3 - / +".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(expected_token, result.unwrap());
    }

    #[test]
    fn expression_with_parentheses_second_expression() {
        let token_list: Vec<String> = "( 5 * 4 + 3 ) - 1".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let result = shunting_yard(token_list);
        assert_eq!(true, result.is_ok());

        let expected_token: Vec<String> = "5 4 * 3 + 1 -".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(expected_token, result.unwrap());
    }

    #[test]
    fn expression_with_parentheses_nested_parentheses() {
        let token_list: Vec<String> = "( 1 + 2 ) * 3 + 4 * ( ( 7 - 5 ) + 6 )".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let result = shunting_yard(token_list);
        assert_eq!(true, result.is_ok());

        let expected_token: Vec<String> = "1 2 + 3 * 4 7 5 - 6 + * +".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(expected_token, result.unwrap());
    }

    #[test]
    fn expression_with_parentheses_nested_parentheses_second_expression() {
        let token_list: Vec<String> = "2 * ( 3 + 5 ) / 4 + 1 * 9 + ( 4 * ( 6 / 3 ) )".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let result = shunting_yard(token_list);
        assert_eq!(true, result.is_ok());

        let expected_token: Vec<String> = "2 3 5 + * 4 / 1 9 * + 4 6 3 / * +".to_owned()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(expected_token, result.unwrap());
    }
}
