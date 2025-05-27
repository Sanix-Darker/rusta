use core::{mem, str};

#[derive(Debug)]
pub enum JsonValue<'a> {
    Null,
    Bool(bool),
    Number(f32),
    String(&'a str),
    Array(Vec<JsonValue<'a>>),
    Object(Vec<(&'a str, JsonValue<'a>)>),
}

pub struct JsonParser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            data: data.as_bytes(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue<'a>, &'static str> {
        self.skip_whitespace();
        match self.peek() {
            b'n' => self.parse_null(),
            b't' | b'f' => self.parse_bool(),
            b'"' => self.parse_string(),
            b'0'..=b'9' | b'-' => self.parse_number(),
            b'[' => self.parse_array(),
            b'{' => self.parse_object(),
            _ => Err("Unexpected token"),
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue<'a>, &'static str> {
        if self.consume(b'n') && self.consume(b'u') && self.consume(b'l') && self.consume(b'l') {
            Ok(JsonValue::Null)
        } else {
            Err("Invalid null")
        }
    }

    // TODO: implement other parse methods...
}
