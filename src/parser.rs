use crate::lexer::Token;

#[derive(Debug)]
pub enum ParsedNode {
    Function {
        name: String,
        params: Vec<ParsedNode>,
        body: Vec<ParsedNode>
    },
    ForLoop {
        count: Vec<ParsedNode>
    },
    WhileLoop {
        condition: Vec<ParsedNode>
    },
    List {
        items: Vec<ParsedNode>
    },
    Dict {
        items: (Box<ParsedNode>, Box<ParsedNode>)
    },
    Int {
        val: Vec<char>
    },
    Float {
        val: Vec<char>
    },
    Str {
        val: String
    },
    Equation {
        items: Vec<Token>
    },
    IfChain {
        conditions: Vec<ParsedNode>,
        bodies: Vec<ParsedNode>
    },
    Comparison {
        operator: Vec<Token>,
        left: Box<ParsedNode>,
        right: Box<ParsedNode>
    },
    Error {
        line_num: usize,
        line: String,
        arrow: String,
        error: String
    }
}

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            position: 0
        }
    }

    pub fn read_token(&mut self) -> &Token {
        let token = &self.tokens[self.position];
        self.position += 1;
        return token;
    }

    pub fn is_for_loop(&mut self) -> bool {
        println!("{:?}", self.tokens[self.position + 1]);

        if self.position + 2 >= self.tokens.len() {
            return false;
        }

        return self.tokens[self.position + 2] == Token::Word(vec!['k', 'a', 's', 't', 'o', 'o'])
    }

    pub fn parse_condition(&mut self, line: Vec<Token>) -> ParsedNode {
        println!("{:?}", line);
        return ParsedNode::Str { val: "b".to_string() }
    }

    pub fn get_for_loop(&mut self) -> ParsedNode {
        let mut first_line: Vec<Token> = vec![];
        while self.position < self.tokens.len() {
            let token = self.read_token();

            if !first_line.contains(&Token::Colon) {
                first_line.push(token.clone());
            }
        }

        let parsed_line = self.parse_condition(first_line);
        println!("{:?}", parsed_line);
        return ParsedNode::Str { val: "What".to_string() }
    }

    pub fn skip_unnecessary(&mut self) {
        loop {
            match self.tokens[self.position] {
                Token::Whitespace(_) | Token::Comment(_) => {
                    self.position += 1;
                }
                _ => { break; }
            }
        }
    }

    pub fn next_node(&mut self) -> ParsedNode {
        self.skip_unnecessary();
        if self.is_for_loop() {
            return self.get_for_loop();
        }

        return ParsedNode::Str { val: "W".to_string() }
    }

    pub fn parse(&mut self) -> Vec<ParsedNode> {
        let mut parsed: Vec<ParsedNode> = vec![];
        while self.position < self.tokens.len() {
            parsed.push(self.next_node());
        }
        return parsed;
    }
}
