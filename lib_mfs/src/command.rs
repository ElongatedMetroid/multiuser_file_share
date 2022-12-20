use crate::{
    error::{MfsError, MfsErrorSource},
    response::MfsResponse, data::Data,
};

pub trait MfsCommand {
    fn execute(&self) -> Result<MfsResponse, Box<dyn std::error::Error>>;
}

pub trait MfsCommandParser {
    fn keyword_parse(keyword: &str) -> Result<Box<dyn MfsCommand>, Box<dyn std::error::Error>>;
    fn parse(s: &str) -> Result<Box<dyn MfsCommand>, Box<dyn std::error::Error>>;
}

pub struct Foo;
impl MfsCommand for Foo {
    fn execute(&self) -> Result<MfsResponse, Box<dyn std::error::Error>> {
        let mut response = MfsResponse::new();

        response.set_message(Some(String::from("Bar")));

        Ok(response)
    }
}
pub struct Bar;
impl MfsCommand for Bar {
    fn execute(&self) -> Result<MfsResponse, Box<dyn std::error::Error>> {
        let mut response = MfsResponse::new();

        response.set_data(Some(Data::from("Wow this text is in a file called Bar!")));
        response.set_message(Some(String::from("Bar")));

        Ok(response)
    }
}

pub struct CommandParser;
impl MfsCommandParser for CommandParser {
    fn keyword_parse(keyword: &str) -> Result<Box<dyn MfsCommand>, Box<dyn std::error::Error>> {
        match keyword {
            "foo" => Ok(Box::new(Foo)),
            "bar" => Ok(Box::new(Bar)),
            _ => Err(Box::new(MfsError {
                source: MfsErrorSource::KeywordNotFound(String::from(keyword)),
            })),
        }
    }

    fn parse(s: &str) -> Result<Box<dyn MfsCommand>, Box<dyn std::error::Error>> {
        let mut tokens = s.split_whitespace();

        let command = Self::keyword_parse(match tokens.next() {
            Some(keyword) => keyword,
            None => {
                return Err(Box::new(MfsError {
                    source: MfsErrorSource::CommandIsEmpty,
                }))
            }
        })?;

        Ok(command)
    }
}
