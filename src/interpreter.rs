use crate::parser::ParsedNode;
use crate::inbuilt::Inbuilt;
use crate::lexer::Token;
use meval::eval_str;

pub struct Interpreter {
}

static mut VARIABLE_DICT: Vec<(String, Box<ParsedNode>)> = Vec::new();
impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, repl: bool, parsed: Vec<ParsedNode>) {
        parsed.iter().for_each(|block| {
            match block {
                ParsedNode::FunctionCall { name, params } => {
                    let out: Vec<ParsedNode> = Inbuilt::new().get_method(name.to_owned())(params.to_vec());

                    if repl && out.len() > 0 {
                        match (&out[0]).to_owned() {
                            ParsedNode::Str { val } => {
                                print!("{:?}\r\n", val);
                            },
                            _ => {
                                self.print((&out[0]).to_owned())
                            }
                        }
                    }
                },
                ParsedNode::Equation { items } => {
                    let output = self.solve_equation(items);
                    self.print(output);
                },
                ParsedNode::Variable { name, exists, value } => {
                    unsafe {
                        let prev = VARIABLE_DICT.iter().position(|(v, _)| v == name);
                        if !exists && value.is_some() {
                            if prev.is_some() {
                                let pos = prev.unwrap();
                                VARIABLE_DICT[pos].1 = value.clone().unwrap();
                            } else {
                                VARIABLE_DICT.push((name.to_owned(), value.clone().unwrap()));
                            }
                        }

                        if *exists {
                            if prev.is_some() {
                                let pos = prev.unwrap();
                                let parsed = VARIABLE_DICT[pos].1.as_ref().clone();

                                match parsed {
                                    ParsedNode::Equation { items } => {
                                        let output = self.solve_equation(&items);
                                        self.print(output);
                                    }
                                    _ => { self.print(parsed) }
                                }
                            }
                        }
                    }
                },
                _ => { self.print(block.to_owned()) }
            }
        });
    }

    pub fn solve_equation(
        &mut self,
        items: &Vec<Token>
    ) -> ParsedNode {
        let mut equation: String = "".to_owned();
        items.into_iter().for_each(|item| {
            match item {
                Token::Int(val) | Token::Float(val) => {
                    let chars: &str = &val.into_iter().collect::<String>();
                    equation += chars;
                },
                Token::Divide => {
                    equation += "/";
                },
                Token::Multiply => {
                    equation += "*";
                },
                Token::Plus => {
                    equation += "+";
                },
                Token::Minus => {
                    equation += "-";
                },
                Token::Modulus => {
                    equation += "%";
                },
                _ => { }
            }
        });

        let output = eval_str(equation).unwrap().to_string();

        if output.contains(".") {
            return ParsedNode::Float {
                val: output.chars().collect()
            }
        }

        return ParsedNode::Int {
            val: output.chars().collect()
        }
    }

    pub fn print(&mut self, node: ParsedNode) {
        match node {
            ParsedNode::List { items } | ParsedNode::Tuple { items } => {
                let mut list: String = "[".to_owned();
                let mut next = ",";
                for x in 0..items.len() {
                    let str_form = &self.get_string((&items[x]).to_owned(), false);
                    list = list + str_form;
                    if x == items.len() - 1 {
                        next = "]";
                        list = list + next;
                        break;
                    }

                    list = list + next + " ";
                }

                if items.len() == 0 {
                    list = "[]".to_owned();
                }

                print!("{}\r\n", list);
            },
            node => { print!("{}", self.get_string(node, true)) }
        }
    }

    pub fn get_string(&mut self, node: ParsedNode, newline: bool) -> String {
        let mut end: &str = "\r\n";
        if !newline { 
            end = "";
        }

        //println!("{:?}", node);
        match node {
            ParsedNode::Function { name, .. } => {
                return format!("Function {}(){}", name, end);
            },
            ParsedNode::Int { val } | ParsedNode::Float { val } => {
                return format!("{}{}", val.into_iter(). collect::<String>(), end);
            },
            ParsedNode::Str { val } => {
                return format!("{}{}", val, end);
            },
            _ => { }
        }

        return "".to_owned();
    }
}
