pub fn shunting_yard(_token: Vec<String>) -> Result<String, String> {
    Ok(String::default())
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
}
