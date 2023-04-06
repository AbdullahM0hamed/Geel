#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    OpenParen,
    CloseParen,
    OpenBrack,
    CloseBrack,
    OpenBrace,
    CloseBrace,
    Colon,
    Assign,
    EOF,
    Comma,
    Plus,
    Minus,
    Divide,
    Multiply,
    Greater,
    GreaterOrEqual,
    Lesser,
    LesserOrEqual,
    Int(Vec<char>),
    Float(Vec<char>),
    Whitespace(char),
    Speech(Vec<char>),
    Comment(Vec<char>),
    Word(Vec<char>)
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_whitespace(ch: char) -> bool {
    let whitespaces = vec![' ', '\t', '\n', '\r'];
    return whitespaces.contains(&ch)
}

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect::<Vec<char>>(),
            position: 0,
            read_position: 0,
            ch: '0'
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '§';
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn get_num(&mut self) -> Token {
        let mut num: Vec<char> = vec![];
        loop {
            if !is_digit(self.ch) && self.ch != '.' {
                break;
            }

            num.push(self.ch);
            self.read_char();
        }

        if num.contains(&'.') {
            return Token::Float(num);
        } else {
            return Token::Int(num);
        }
    }

    pub fn get_word(&mut self) -> Token {
        let mut word: Vec<char> = vec![];
        loop {
            if !is_letter(self.ch) {
                break;
            }

            word.push(self.ch);
            self.read_char();
        }

        return Token::Word(word);
    }

    pub fn is_comment(&mut self) -> bool {
        if self.ch == '/' && self.read_position < self.input.len() {
            let next = self.input[self.read_position];
            return next == '/' || next == '*';
        } else {
            return false;
        }
    }

    pub fn get_comment(&mut self) -> Token {
        let next = self.input[self.read_position];
        let end = if next == '/' { "\n" } else { "*/" };
        let mut comment: Vec<char> = vec![];

        loop {
            if comment.len() >= end.len() && comment[comment.len() - end.len()..comment.len()] == end.chars().collect::<Vec<char>>() || self.read_position > self.input.len() {
                break;
            }

            comment.push(self.ch);
            self.read_char();
        }

        return Token::Comment(comment);
    }

    pub fn is_speech(&mut self) -> bool {
        return (self.ch == '"' || self.ch == '\'') && (self.position == 0 || self.input[self.position - 1] != '\\')
    }

    pub fn get_speech(&mut self) -> Token {
        let mut speech: Vec<char> = vec![];
        let end = self.ch;
        speech.push(self.ch);
        self.read_char();

        loop {
            if self.ch == end && self.input[self.position - 1] != '\\' {
                speech.push(self.ch);
                self.read_char();
                break;
            }

            speech.push(self.ch);
            self.read_char();
        }

        return Token::Speech(speech);
    }

    pub fn next_token(&mut self) -> Token {
        let token;

        if is_letter(self.ch) {
            let a = self.get_word();
            return a;
        }

        if is_digit(self.ch) {
            return self.get_num();
        }

        if is_whitespace(self.ch) {
            token = Token::Whitespace(self.ch);
            self.read_char();
            return token;
        }

        if self.is_comment() {
            return self.get_comment();
        }

        if self.ch == '"' {
            return self.get_speech();
        }

        if self.ch == '>' && self.read_position > self.input.len() && self.input[self.read_position] == '=' {
            return Token::GreaterOrEqual;
        }

        if self.ch == '<' && self.read_position > self.input.len() && self.input[self.read_position] == '=' {
            return Token::LesserOrEqual;
        }

        match self.ch {
            '(' => { token = Token::OpenParen; }
            ')' => { token = Token::CloseParen; }
            '[' => { token = Token::OpenBrack; }
            ']' => { token = Token::CloseBrack; }
            '{' => { token = Token::OpenBrace; }
            '}' => { token = Token::CloseBrace; }
            ':' => { token = Token::Colon; }
            ',' => { token = Token::Comma; }
            '=' => { token = Token::Assign; }
            '+' => { token = Token::Plus; }
            '-' => { token = Token::Minus; }
            '/' => { token = Token::Divide; }
            '*' => { token = Token::Multiply; }
            '>' => { token = Token::Greater; }
            '<' => { token = Token::Lesser; }
            '§' => { token = Token::EOF; }
            _ => { token = Token::Comma; }
        }
        self.read_char();
        token
    }

    pub fn lex(&mut self) -> Vec<Token> {
        self.read_char();
        let mut tokens: Vec<Token> = vec![];
        loop {
            let token = self.next_token();
            if token == Token::EOF {
                break;
            } else {
                tokens.push(token);
            }
        }

        tokens
    }
}
