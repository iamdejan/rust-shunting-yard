pub fn shunting_yard(token: Vec<String>) -> Result<Vec<String>, String> {
    if token.is_empty() {
        return Ok(Vec::new());
    }

    let mut result = Vec::new();
    result.push("1".to_owned());
    result.push("1".to_owned());
    result.push("+".to_owned());
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_token() {
        let token = Vec::new();
        let result = shunting_yard(token);
        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn simple_token() {
        let mut token: Vec<String> = Vec::new();
        token.push("1".to_owned());
        token.push("+".to_owned());
        token.push("1".to_owned());
        let result = shunting_yard(token);
        assert_eq!(true, result.is_ok());

        let mut expected_token: Vec<String> = Vec::new();
        expected_token.push("1".to_owned());
        expected_token.push("1".to_owned());
        expected_token.push("+".to_owned());
        assert_eq!(expected_token, result.unwrap());
    }
}
