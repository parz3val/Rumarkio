use std::{
    fmt::{format, Error},
    str::FromStr,
};

/// Serialized Format email|name
#[derive(Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
}

pub trait Json {
    fn json(&self) -> Result<String, Error>;
}

impl Json for SignUpRequest {
    fn json(&self) -> Result<String, Error> {
        let mut json_str = String::from("");
        json_str.push('{');

        // email
        json_str.push('"');
        json_str.push_str("email");
        json_str.push('"');

        json_str.push('"');
        json_str.push_str(self.email.as_str());
        json_str.push('"');

        // password
        json_str.push('"');
        json_str.push_str("password");
        json_str.push('"');

        json_str.push('"');
        json_str.push_str(self.password.as_str());
        json_str.push('"');

        json_str.push('}');
        Ok(json_str)
    }
}

// Gave up on json parsing lol
// pub struct JsonParser {
// }

impl FromStr for SignUpRequest {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("|").collect();
        if parts.len() > 1 {
            return Ok(SignUpRequest {
                email: parts[0].to_string(),
                password: parts[1].to_string(),
            });
        }
        return Err("Failed to parse signup request");
    }
}

#[cfg(test)]
mod abstracts_tests {
    use super::*;

    #[test]
    fn test_user_sign_up_serialization() {
        let my_user = "email@email.com|123,";
        let sign_up_request = SignUpRequest::from_str(my_user).unwrap();
        assert_eq!(
            sign_up_request.email,
            SignUpRequest {
                email: String::from("email@email.com"),
                password: String::from("123,"),
            }
            .email
        );
    }
}
