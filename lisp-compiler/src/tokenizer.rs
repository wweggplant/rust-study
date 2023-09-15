use regex::Regex;

/* 
  tokenizer.rs
*/

/* 
  一个结构体，用来存储 token
  {
    type: string,
    value: string
  }
*/
#[derive(Debug, PartialEq)]  // 添加PartialEq特性
pub enum TokenType {
    Number,
    Paren,
    Name,
    // 可以添加其他标记类型
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }
}
// a regex for letters
pub fn tokenizer(input: &str) -> Vec<Token> {
  let letter_regex = Regex::new(r"[a-z]").unwrap();
  let white_regex = Regex::new(r"\s").unwrap();
  let number_regex = Regex::new(r"\d").unwrap();

  let mut tokens = Vec::new();
  let chars: Vec<char> = input.chars().collect(); // 转换为字符数组
  let mut current = 0;

  while current < chars.len() {
      let char = chars[current];

      // ...
      if char == '(' || char == ')' {
          tokens.push(
              Token {
                  token_type: TokenType::Paren,
                  value: char.to_string(),
              }
          );
          current += 1;
          continue;
      }

      // letter token
      if letter_regex.is_match(&char.to_string()) {
          let mut value = String::new();

          while current < chars.len() && letter_regex.is_match(&chars[current].to_string()) {
              value.push(chars[current]);
              current += 1;
          }

          tokens.push(
              Token {
                  token_type: TokenType::Name,
                  value: value,
              }
          );
          continue;
      }
      if white_regex.is_match(&char.to_string()) {
          current += 1;
          continue;
      }
      if number_regex.is_match(&char.to_string()) {
          let mut value = String::new();

          while current < chars.len() && number_regex.is_match(&chars[current].to_string()) {
              value.push(chars[current]);
              current += 1;
          }

          tokens.push(
              Token {
                  token_type: TokenType::Number,
                  value: value,
              }
          );
          continue;
      }
      panic!("Unknown char: {:?}", char);
  }
  // ... 
  tokens
}