use crate::lexer::Token;

#[derive(Debug)]
#[derive(Clone)]
pub enum ParsedNode {
    Function {
        name: String,
        params: Vec<ParsedNode>,
        body: Vec<ParsedNode>
    },
    FunctionCall {
        name: String,
        params: Vec<ParsedNode>
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
    Tuple {
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
        blocks: Vec<(Vec<Vec<ParsedNode>>, Vec<ParsedNode>)>
    },
    Comparison {
        operator: Token,
        left: Box<Vec<ParsedNode>>,
        right: Box<Vec<ParsedNode>>
    },
    Error {
        line_num: usize,
        line: String,
        arrow: String,
        error: String
    },
    Variable {
        name: String,
        exists: bool,
        add_sub: usize,
        value: Option<Box<ParsedNode>>
    },
    Ignore
}

const KEYWORDS: [&str; 103] = [
    "iyo", "maaha", "ama", "gudub", "booliyan", "jooji", "Run", "Been",
    "Waxba", "keen", "ka", "sida", "tijaabi", "qabo", "ugu", "dambeyn",
    "xaqiiji", "kayd", "qayb", "tir", "hadduu", "haddii", "kale", "kastoo",
    "caalami", "kujira", "waa", "laamda", "dhaaf", "Tus", "celi", "intuu",
    "isticmaal", "sii", "qiimahasugan", "kulli", "midkasta", "labaale",
    "bool", "qaybkaydeed", "dhis", "qaamuus", "sifosheeg", "qaybiyobaaq",
    "tiri", "qiimee", "bax", "kasooc", "tobanle", "hagaaji", "caalamiyaasha",
    "sifomaleeyahay", "caawimaad", "lixyatobaneyn", "lambarugaar", "weydii",
    "tirodhan", "midmid", "dherer", "aruur", "ugubadnaan", "uguyaraan", "wad",
    "wax", "siddeedid", "fur", "qor", "sifo", "faraq", "muuqaal", "rogan",
    "tirobuuxin", "urur", "qaybi", "soocan", "qoraal", "iskudar", "dhaxal",
    "uruur", "nooc", "iskuxer", "KhaladAasaasi", "Khalad", "KhaladXisaabeed",
    "KhaladXaqiijin", "KhaladSifeed", "KhaladQoraalDhamaa", "KhaladTobanle",
    "KhaladKeenid", "KhaladJagaale", "WaaLaJoojiyey", "KhaladXasuuseed",
    "KhaladMagceed", "KhaladLamaSameyn", "KhaladCelcelis", "NoocKhaldan",
    "KhaladQiimeyn", "KhaladEberUQeybin", "KhaladXiriixLaGoo", "KhaladXiriixLaDiid",
    "KhaladOgolaansho", "DigniinKeenid", "markuu"
];

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

    pub fn skip_unnecessary(
        &mut self,
        tokens: Vec<Token>,
        pos: usize
    ) -> usize {
        let mut position = pos;
        loop {
            if position >= tokens.len() {
                return position;
            }

            match tokens[position] {
                Token::Whitespace(_) | Token::Comment(_) => {
                    position += 1;
                }
                _ => { break; }
            }
        }

        return position;
    }

    pub fn get_function_call(
        &mut self,
        word: Vec<char>,
        tokens: Vec<Token>,
        pos: usize
    ) -> (ParsedNode, usize) {
        let mut position = pos + 1;
        let name: String = word.into_iter().collect();
        let mut open_count: usize = 0;
        let mut close_count: usize = 0;
        let mut args: Vec<Vec<Token>> = Vec::new();
        let mut arg_pos: usize = 0;
        loop {
            position = self.skip_unnecessary(
                (&tokens).to_owned(),
                (&position).to_owned()
            );
            if position >= tokens.len() {
                break;
            }

            let token = &tokens[position];
            position += 1;
            if token == Token::OpenParen {
                open_count += 1;
            } else if token == Token::CloseParen {
                if (close_count + 1) != open_count {
                    close_count += 1;
                } else {
                    break;
                }
            } else if token == Token::Comma {
                arg_pos += 1;
            } else {
                if args.len() != arg_pos + 1 {
                    args.push(vec![token.clone()]);
                } else {
                    args[arg_pos].push(token.clone());
                }
            }
        }

        let mut params: Vec<ParsedNode> = vec![];
        args.iter().for_each(|item| {
            if item.len() == 1 {
                match &item[0] {
                    Token::Int(value) => {
                        params.push(
                            ParsedNode::Int { val: value.to_vec() }
                            );
                    },
                    Token::Float(value) => {
                        params.push(
                            ParsedNode::Float { val: value.to_vec() }
                            );
                    },
                    Token::Speech(value) => {
                        params.push(
                            ParsedNode::Str { val: value.into_iter().collect() }
                            );
                    },
                    _ => { }
                }
            }
        });

        return (ParsedNode::FunctionCall {
            name: name,
            params: params
        }, position)
    }

    const OPERATIONS: [Token; 5] = [
        Token::Plus,
        Token::Minus,
        Token::Divide,
        Token::Multiply,
        Token::Modulus
    ];

    pub fn get_num_or_parse(
        &mut self,
        custom: bool,
        token_list: Option<Vec<Token>>,
        pos: Option<usize>
        ) -> (ParsedNode, usize) {
        let mut tokens = (&self.tokens).to_owned();
        let mut position = (&self.position).to_owned();

        if custom && token_list.is_some() && pos.is_some() {
            tokens = token_list.unwrap();
            position = pos.unwrap();
        }

        let token = (&tokens[position]).to_owned();

        //TODO: Make this work as a method
        let mut next = position + 1;
        loop {
            if next >= tokens.len() {
                next -= 1;
                break;
            }

            match &tokens[next] {
                Token::Whitespace(_) | Token::Comment(_) => {
                    next += 1;
                },
                _ => { break; }
            }
        }

        let next = (&tokens[next]).to_owned();
        if Self::OPERATIONS.contains(&next) {
            let parsed =  self.parse_expression(tokens, position);

            if !custom {
                self.position = parsed.1;
            }

            return parsed;
        } else {
            position += 1;
            if !custom {
                self.position = position;
            }

            match token {
                Token::Int(val) => {
                    return (ParsedNode::Int { val: val }, position);
                },
                Token::Float(val) => {
                    return (ParsedNode::Float { val: val }, position);
                },
                _ => { }
            }
        }

        if !custom {
            self.position = position;
        }

        return (ParsedNode::Ignore, position);
    }

    pub fn operation_end(
        &mut self,
        tokens: Vec<Token>,
        position: usize
        ) -> usize {
        let mut next = position + 1;
        loop {
            if next >= tokens.len() {
                break;
            }

            let token = (&tokens[next]).to_owned();
            match token {
                Token::Whitespace(_) | Token::Comment(_) => {
                    next += 1;
                },
                Token::Plus | Token::Minus | Token::Divide | Token::Multiply | Token::Modulus => {
                    next += 1;
                },
                Token::Int(_) | Token::Float(_) => {
                    next += 1;
                },
                _ => {
                    break;
                }
            }
        }

        return next;
    }

    pub fn parse_expression(
        &mut self,
        tokens: Vec<Token>,
        pos: usize
        ) -> (ParsedNode, usize) {
        let mut position = pos;
        let mut operation_list: Vec<Token> = vec![];
        let end = self.operation_end(tokens.clone(), pos);

        loop {
            if position <= end && position < tokens.len() {
                let token = (&tokens[position]).to_owned();
                match token {
                    Token::Whitespace(_) | Token::Comment(_) => {
                        position += 1;
                    },
                    _ => {
                        operation_list.push(token);
                        position += 1;
                    }
                }
            } else {
                break;
            }
        }

        return (ParsedNode::Equation { items: operation_list }, position);
    }

    pub fn is_still_if(&mut self) -> bool {
        let mut non_whitespace: usize = self.position;
        loop {
            if non_whitespace >= self.tokens.len() {
                non_whitespace -= 1;
                break;
            }

            match &self.tokens[non_whitespace] {
                Token::Whitespace(_) | Token::Comment(_) => {
                    non_whitespace += 1;
                },
                _ => { break; }
            }
        }

        if non_whitespace < self.tokens.len() {
            let token = (&self.tokens[non_whitespace]).to_owned();
            match token {
                Token::Word(word) => {
                    if &word.iter().collect::<String>() == "haddii" || &word.iter().collect::<String>() == "ama" {
                        return true;
                    }
                }
                _ => { }
            }
        }

        return false;
    }

    pub fn get_if_parsed(&mut self) -> ParsedNode {
        let mut indent_level = 0;

        if self.position > 1 {
            match (&self.tokens[self.position - 1]).to_owned() {
                Token::Whitespace(space) => {
                    if space[space.len() - 1] != '\n' {
                        let pos = space.iter().position(|&n| n == '\n').unwrap();
                        indent_level = space.len() - (pos + 1);
                    }
                },
                _ => { }
            }
        }

        let mut if_tokens: Vec<(Vec<Token>, usize)> = Vec::new();
        let mut current_indent: usize = indent_level;
        let mut if_pos: usize = 0;
        loop {
            if self.position >= self.tokens.len() {
                break;
            }

            match (&self.tokens[self.position]).to_owned() {
                Token::EOF => { 
                    self.position += 1;
                    break;
                }
                Token::Whitespace(space) => {
                    let has_newline = space.iter().position(|&n| n == '\n');
                    if !has_newline.is_none() {
                        let pos = has_newline.unwrap();
                        let level = space.len() - (pos + 1);
                        if current_indent != level {
                            current_indent = level;
                            if_pos += 1;
                        }

                        if level == indent_level {
                            if self.is_still_if() {
                                let token = (&self.tokens[self.position]).to_owned();
                                if if_pos >= if_tokens.len() {
                                    if_tokens.push((vec![token], current_indent));
                                } else {
                                    if_tokens[if_pos].0.push(token);
                                }
                                self.position += 1;
                                continue;
                            }

                            break;
                        }

                    }
                    self.position += 1;
                }
                x => {
                    if if_pos >= if_tokens.len() {
                        if_tokens.push((vec![x], current_indent));
                    } else {
                        if_tokens[if_pos].0.push(x);
                    }

                    self.position += 1;
                }
            }
        }

        let a = self.parse_conditions(if_tokens);
        //print!("M: {:?}\r\n", a);
        return a;
    }

    pub fn parse_conditions(
        &mut self,
        conditions: Vec<(Vec<Token>, usize)>
        ) -> ParsedNode {
        let mut parsed_conditions: Vec<(Vec<Vec<ParsedNode>>, Vec<ParsedNode>)> = Vec::new();

        let if_indent = conditions[0].1;
        for cond in conditions {
            let indent_level = cond.1;

            if indent_level == if_indent {
                let mut parsed_cond: Vec<Vec<ParsedNode>> = Vec::new();
                let or_divisions: Vec<Vec<Vec<Token>>> = self.get_or_separated((&cond.0).to_owned());
                for or_list in or_divisions {
                    let mut combined: Vec<ParsedNode> = Vec::new();
                    for and_cond in or_list {

                        let assign = and_cond.iter().rev().position(|r| r == Token::Assign);
                        let mut cond_type = Token::Ignore;
                        if assign.is_some() {
                            let assign_index = and_cond.len() - assign.unwrap() - 1;
                            let prev_token = (&and_cond[assign_index - 1]).to_owned();

                            match prev_token {
                                Token::Assign => {
                                    cond_type = Token::Equal;
                                }
                                Token::Less => {
                                    cond_type = Token::LessOrEqual;
                                }
                                Token::Greater => {
                                    cond_type = Token::GreaterOrEqual;
                                }
                                _ => { }
                            }
                        }

                        if cond_type != Token::Ignore {
                            let assign_index = and_cond.len() - assign.unwrap() - 1;
                            let mut comp_one: Vec<Token> = Vec::new();
                            let mut comp_two: Vec<Token> = Vec::new();
                            for (index, token) in and_cond.iter().enumerate() {
                                if index < assign_index - 1 {
                                    comp_one.push(token.clone());
                                } else if index > assign_index {
                                    comp_two.push(token.clone());
                                }
                            }

                            let parsed_one = self.get_parsed_comp(comp_one);
                            let parsed_two = self.get_parsed_comp(comp_two);
                            let comparison  = ParsedNode::Comparison {
                                operator: cond_type,
                                left: Box::new(parsed_one.clone()),
                                right: Box::new(parsed_two.clone())
                            };

                            combined.push(comparison);
                        }
                    }

                    parsed_cond.push(combined);
                }

                parsed_conditions.push((parsed_cond, Vec::new()));
            } else {
                let node = self.next_node(true, Some(cond.0.to_owned()), Some(0));
                let last = parsed_conditions.len() - 1;
                parsed_conditions[last].1.push(node);
            }
        }

        return ParsedNode::IfChain { blocks: parsed_conditions };
    }

    pub fn get_or_separated(
        &mut self,
        tokens: Vec<Token>
        ) -> Vec<Vec<Vec<Token>>> {
        let or_split = tokens
            .split(|&ref v| v == Token::Word(vec!['a', 'm', 'a']));

        let mut splits: Vec<Vec<Vec<Token>>> = Vec::new();
        for or in or_split {
            let and_split = or
                .split(|v| v == Token::Word(vec!['i', 'y', 'o']));

            let mut normal_and_split: Vec<Vec<Token>> = Vec::new();
            for and in and_split {
                normal_and_split.push(and.to_owned());
            }

            splits.push(normal_and_split);
        }

        return splits;
    }

    pub fn get_parsed_comp(
        &mut self,
        comp: Vec<Token>
        ) -> Vec<ParsedNode> {
        let mut parsed_comp: Vec<ParsedNode> = Vec::new();
        let mut iter = comp.iter().enumerate();

        while let Some(comp_part) = iter.next() {
            let index = comp_part.0;
            let token = comp_part.1;
            match token {
                Token::Word(word) => {
                    let word_str = word.iter().collect::<String>();
                    if !KEYWORDS.contains(&(word_str.as_str())) {
                        parsed_comp.push(
                            ParsedNode::Variable {
                                name: word_str.to_owned(),
                                exists: true,
                                add_sub: 0,
                                value: None
                            }
                        );
                    }
                }
                Token::Int(_) | Token::Float(_) => {
                    let parsed = self.get_num_or_parse(
                        true,
                        Some(comp.clone()),
                        Some(index)
                        );
                    parsed_comp.push(parsed.0);
                    iter.nth(parsed.1 - index - 1);
                    continue;
                }
                _ => { }
            }
        }

        return parsed_comp;
    }

    pub fn next_node(
        &mut self,
        custom: bool,
        token_list: Option<Vec<Token>>,
        pos: Option<usize>
    ) -> ParsedNode {
        let mut tokens = (&self.tokens).to_owned();
        let mut position = (&self.position).to_owned();

        if custom && token_list.is_some() && pos.is_some() {
            tokens = token_list.unwrap();
            position = pos.unwrap();
        }

        position = self.skip_unnecessary(tokens.clone(), position);
        let mut next = position + 1;
        loop {
            if next >= tokens.len() {
                next -= 1;
                break;
            }

            match &tokens[next] {
                Token::Whitespace(_) | Token::Comment(_) => {
                    next += 1;
                },
                _ => { break; }
            }
        }

        if position >= tokens.len() {
            return ParsedNode::Ignore;
        }

        let mut node = ParsedNode::Ignore;
        match &tokens[position] {
            Token::Word(word) => {
                let word_str = word.to_vec().iter().collect::<String>();
                if &tokens[next] == Token::OpenParen {
                    let func = self.get_function_call(word.to_vec(), tokens.clone(), position.clone());
                    node = func.0;
                    position = func.1;
                } else if &word_str == "hadduu" {
                    node = self.get_if_parsed();
                } else if self.is_assignment(tokens.clone(), position) {
                    let assigned = self.get_assignment(tokens, position);
                    position = assigned.1;
                    node = assigned.0;
                } else if !KEYWORDS.contains(&(word_str.as_str())) {
                    //For loop stuff
                    node = ParsedNode::Variable {
                        name: word_str,
                        exists: true,
                        add_sub: 0,
                        value: None
                    };
                    position += 1;
                } else {
                    position += 1;
                }
            },
            Token::Int(_) => {
                node = self.get_num_or_parse(custom, Some(tokens), Some(position)).0;
            },
            Token::Float(_) => {
                node = self.get_num_or_parse(false, None, None).0;
            },
            Token::Speech(val) => {
                position += 1;
                node = ParsedNode::Str { val: val.into_iter().collect::<String>() }
            },
            _ => { position += 1 }
        }

        if !custom {
            if self.position < position {
                self.position = position;
            }
        }

        return node;
    }

    pub fn is_assignment(
        &mut self,
        tokens: Vec<Token>,
        position: usize
    ) -> bool {
        let sliced_tokens = &tokens[position..tokens.len()];

        return sliced_tokens.contains(&Token::Assign);
    }

    pub fn get_assignment(
        &mut self,
        tokens: Vec<Token>,
        position: usize
    ) -> (ParsedNode, usize) {
        let sliced_tokens = &tokens[position..tokens.len()];
        let equal = sliced_tokens.iter().position(|pos| pos == Token::Assign);

        let mut add_sub = 0;
        if equal.is_some() {
            match &tokens[&equal.unwrap() - 1] {
                Token::Plus => {
                    add_sub = 1;
                }
                Token::Minus => {
                    add_sub = 2;
                }
                _ => { }
            }

            let name_map = sliced_tokens.iter().enumerate().filter(|(i, _)| i < &equal.unwrap()).map(|(_, v)| v);

            let mut name: String = "".to_owned();
            for token in name_map {
                match token {
                    Token::Word(word) => {
                        name = word.iter().collect::<String>();
                    }
                    _ => { }
                }
            }

            let value_map = sliced_tokens.iter().enumerate().filter(|(i, _)| i > &equal.unwrap()).map(|(_, v)| v);
            let mut value: Vec<Token> = Vec::new();
            for v in value_map {
                value.push(v.clone());
            }

            return (
                ParsedNode::Variable {
                    name: name,
                    exists: false,
                    add_sub: add_sub,
                    value: Some(
                        Box::new(self.next_node(
                            true,
                            Some(value),
                            Some(0)
                        ))
                    )
                },
                position + sliced_tokens.len()
            );
        }

        return (ParsedNode::Ignore, position + 1);
    }

    pub fn parse(&mut self) -> Vec<ParsedNode> {
        let mut parsed: Vec<ParsedNode> = vec![];
        let mut prev: usize = 0;
        while self.position < self.tokens.len() {
            //TODO: Find out what keeps making it loop infinitely  in if statements
            if self.position > 0 && self.position == prev {
                break;
            }

            prev = self.position;
            parsed.push(self.next_node(false, None, None));
        }
        return parsed;
    }
}
