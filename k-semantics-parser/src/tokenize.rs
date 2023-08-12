use plex::lexer;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Token {
    Ident(String),
    Str(String),
    Number(u64),
    Implication,
    OpenParen,
    OpenSquareParen,
    OpenCurlyParen,
    CloseParen,
    CloseSquareParen,
    CloseCurlyParen,
    Comma,
    If,
    Then,
    Else,
    Fi,
    Whitespace,
    Equal,
    ColonGreater,
}

pub struct Lexer {
    data: String,
}

impl Lexer {
    pub fn new(data: String) -> Self {
        Self {
            data,
        }
    }

    /*fn tokenize(s: &str) -> Result<Option<(Token,&str)>, _>{
        let mut chars = s.chars();
        match chars.next()? {
            '0'..='9' =>{
                todo!()
            }
            'a'..='z'| 'A'..='Z' | '_' => {
                todo!()
            }
            '#' => {
                todo!()
            }
            '|' => {
                todo!()
            }
            '('=> Ok(Some((Token::OpenParen, &s[1..]))),
            ')'=> Ok(Some((Token::CloseParen, &s[1..]))),
            ',' => Ok(Some((Token::Comma, &s[1..]))),
            _ => {
                todo!()
            }
        }
    }*/

    pub fn tokens(self) -> Vec<Token> {
        let mut str = self.data.as_str();
        let mut res = vec![];
        loop {
            match Self::take_token(str) {
                Some((token, next_str)) => {
                    res.push(token);
                    str = next_str;
                }
                None => {
                    assert!(str.is_empty());
                    return res;
                }
            }
        }
    }
    lexer! {
        fn take_token(tok: 'a) -> Token;
        "[0-9]+" => Token::Number(tok.parse().unwrap()),
        "[A-Za-z_][A-Za-z_0-9]+" => Token::Ident(tok.to_string()),
        "\"[A-Za-z_][A-Za-z_0-9]+\"" => Token::Str(tok.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string()),
        "\\|->" => Token::Implication,
        "," => Token::Comma,
        "\\(" => Token::OpenParen,
        "\\[" => Token::OpenSquareParen,
        "\\{" => Token::OpenCurlyParen,
        "\\)" => Token::CloseParen,
        "\\]" => Token::CloseSquareParen,
        "\\}" => Token::CloseCurlyParen,
        ":>" => Token::ColonGreater,
        "#ifMInt" => Token::If,
        "#then" => Token::Then,
        "#else" => Token::Else,
        "#fi" => Token::Fi,
        "==Bool" => Token::Equal,
        "(\\t| |\\n)+" => Token::Whitespace,
    }
}

pub fn remove_whitespace(input: Vec<Token>) -> Vec<Token> {
    input.into_iter().filter(|token|!matches!(token, Token::Whitespace)).collect()
}



