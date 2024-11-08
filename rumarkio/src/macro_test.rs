#[cfg(test)]
mod test {
    use rumarkio::AnswerFn;
    use rumarkio::Jsonizer;
    use crate::abstracts::Json;

    #[derive(AnswerFn)]
    struct Struct;

    #[derive(Jsonizer)]
    pub struct TestJson {
        pub name: String,
        pub email: String,
    }

    #[test]
    fn test() {
        let json_obj = TestJson {
            name: "e@a.com".to_string(),
            email: "email@email.com".to_string(),
        };
        let json_resp = json_obj.json();
        assert_eq!(json_resp, Ok(String::from("hello")));
    }
}
