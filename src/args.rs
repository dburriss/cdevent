use std::error::Error;

pub fn parse_key_val(s: &str) -> Result<(String, String), Box<dyn Error + Send + Sync + 'static>>
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

pub fn parse_comma_delimited(s: &str, error_message: &str) -> Result<Vec<(String, String)>, Box<dyn Error + Send + Sync + 'static>>
{
    if s.is_empty() {
        return Err(error_message.into());
    }
    let r = s.split(',').map(|kv| parse_key_val(kv).unwrap()).collect();
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_key_val_when_empty_string_then_error(){
        let result: Result<(String, String), Box<dyn Error+Send+Sync>> = parse_key_val("");
        assert!(result.is_err());
    }

    #[test]
    fn parse_key_val_when_equal_sign_then_key_value(){
        let result: Result<(String, String), Box<dyn Error+Send+Sync>> = parse_key_val("x=y");
        assert_eq!(result.unwrap(), (String::from("x"), String::from("y")))
    }

    #[test]
    fn parse_custom_data_when_empty_then_error(){
        let result: Result<Vec<(String, String)>, Box<dyn Error+Send+Sync>> = parse_comma_delimited("", "error");
        assert!(result.is_err())
    }

    #[test]
    fn parse_custom_data_when_single_key_value_then_key_value(){
        let result: Result<Vec<(String, String)>, Box<dyn Error+Send+Sync>> = parse_comma_delimited("key1=value1", "error");
        assert_eq!(result.unwrap(), vec![(String::from("key1"), String::from("value1"))])
    }

    #[test]
    fn parse_custom_data_when_multiple_key_value_then_key_value(){
        let result: Result<Vec<(String, String)>, Box<dyn Error+Send+Sync>> = parse_comma_delimited("key1=value1,key2=value2", "error");
        assert_eq!(result.unwrap(), vec![(String::from("key1"), String::from("value1")), (String::from("key2"), String::from("value2"))])
    }
}
