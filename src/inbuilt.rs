use crate::parser::ParsedNode;
use crate::interpreter::VARIABLE_DICT;

pub struct Inbuilt<'a> {
    pub methods: Vec<(String, &'a dyn Fn(Vec<ParsedNode>) -> Vec<ParsedNode>)>
}

impl Inbuilt<'_> {
    pub fn stub(_params: Vec<ParsedNode>) -> Vec<ParsedNode> { return vec![]; }

    pub fn new() -> Self {
        Self {
            methods: vec![
                (
                    "qor".to_string(),
                    &Self::qor
                ),
                (
                    "labaale".to_string(),
                    &Self::labaale
                ),
                (
                    "qaybiyobaaq".to_string(),
                    &Self::qaybiyobaaq
                ),
                (
                    "faraq".to_string(),
                    &Self::faraq
                )
            ]
        }
    }

    pub fn get_method(
        &mut self,
        name: String
    ) -> &'_ dyn Fn(Vec<ParsedNode>) -> Vec<ParsedNode> {
        let mut func: &'_ dyn Fn(Vec<ParsedNode>) -> Vec<ParsedNode> = &Self::stub;
        self.methods.iter().for_each(|method| {
            if method.0 == name {
                func = method.1;
            }
        });

        return func;
    }

    pub fn qor(params: Vec<ParsedNode>) -> Vec<ParsedNode> { 
        let mut output: String = "".to_owned();
        params.iter().for_each(|arg| {
            match arg {
                ParsedNode::Variable { name, .. } => {
                    unsafe {
                        let var = VARIABLE_DICT.iter().position(|(v, _)| v == name);
                        if var.is_some() {
                            let loc = var.unwrap();
                            let val = &VARIABLE_DICT[loc];
                            output.push_str(
                                &Self::parsed_string(val.1.as_ref().clone())
                            );
                        }
                    }
                },
                x => { output.push_str(&Self::parsed_string(x.clone())) }
            }
        });

        let finalised = output.trim();
        if finalised.len() > 0 && ['"', '\''].contains(&finalised.chars().nth(0).unwrap()) && finalised.chars().nth(finalised.len() - 1) == finalised.chars().nth(0) {
            let to_print: String = finalised.chars().enumerate().filter(|(x, _)| x != &(0 as usize) && x != &(finalised.len() - 1)).map(|(_, v)| v).collect();
            print!("{}\r\n", to_print.trim());
        } else {
            print!("{}\r\n", output.trim());
        }

        return vec![];
    }

    pub fn parsed_string(
        value: ParsedNode
    ) -> String {
        match value {
                ParsedNode::Str { val } => {
                    return val;
                },
                ParsedNode::Int { val } | ParsedNode::Float { val } => {
                    return val.iter().collect::<String>();
                }
                _ => { }
        }

        return "".to_owned();
    }
    pub fn labaale(params: Vec<ParsedNode>) -> Vec<ParsedNode> {
        let arg: ParsedNode = (&params[0]).to_owned();
        match arg {
            ParsedNode::Int { val } => {
                let string: String = val.into_iter().collect::<String>();
                let int: i32 = string.parse::<i32>().unwrap();
                return vec![ParsedNode::Str { val: format!("0b{:b}", int) }];
            },
            _ => { }
        }

        return vec![];
    }

    pub fn qaybiyobaaq(params: Vec<ParsedNode>) -> Vec<ParsedNode> {
        if params.len() == 2 {
            let arg_one = (&params[0]).to_owned();
            let arg_two = (&params[1]).to_owned();

            let mut arg_is_float: bool = false;
            let arg_one_val: Vec<char>;
            let arg_two_val: Vec<char>;

            match arg_one {
                ParsedNode::Float { val } => {
                    arg_is_float = true;
                    arg_one_val = val;
                },
                ParsedNode::Int { val } => {
                    arg_one_val = val;
                },
                _ => { return vec![]; }
            }

            match arg_two {
                ParsedNode::Float { val } => {
                    arg_is_float = true;
                    arg_two_val = val;
                },
                ParsedNode::Int { val } => {
                    arg_two_val = val;
                },
                _ => { return vec![]; }
            }

            let param_one: String = arg_one_val.into_iter().collect::<String>();
            let param_two: String = arg_two_val.into_iter().collect::<String>();

            if arg_is_float {
                let float_one: f64 = param_one.parse::<f64>().unwrap();
                let float_two: f64 = param_two.parse::<f64>().unwrap();
                let quotient: f64 = float_one / float_two;
                let remainder: f64 = float_one % float_two;

                return vec![
                    ParsedNode::Tuple {
                        items: vec![
                            ParsedNode::Float { val: quotient.to_string().chars().collect() },
                            ParsedNode::Float { val: remainder.to_string().chars().collect() }
                        ]
                    }
                ];
            } else {
                let int_one: i32 = param_one.parse::<i32>().unwrap();
                let int_two: i32 = param_two.parse::<i32>().unwrap();
                let quotient: i32 = int_one / int_two;
                let remainder: i32 = int_one % int_two;

                return vec![
                    ParsedNode::Tuple {
                        items: vec![
                            ParsedNode::Int { val: quotient.to_string().chars().collect() },
                            ParsedNode::Int { val: remainder.to_string().chars().collect() }
                        ]
                    }
                ];
            }
        }

        return vec![];
    }

    pub fn faraq(params: Vec<ParsedNode>) -> Vec<ParsedNode> {
        if params.len() != 2 {
            println!("NoocKhaldan: faraq() 2 shay buu qaataa, laakiin {} shay baa la siiyay", params.len());
        } else {
            let mut one: i32 = 0;
            let two: i32;

            let param_one = (&params[0]).to_owned();
            let mut one_is_int: bool = false;
            match param_one {
                ParsedNode::Int { val } => {
                    let string_one = val.into_iter().collect::<String>();
                    one = string_one.parse::<i32>().unwrap();
                    one_is_int = true;
                },
                _ => { }
            }

            if one_is_int {
                let param_two = (&params[1]).to_owned();
                match param_two {
                    ParsedNode::Int { val } => {
                        let string_two = val.into_iter().collect::<String>();
                        two = string_two.parse::<i32>().unwrap();
                        let mut range: Vec<ParsedNode> = vec![];
                        (one..two).for_each(|item| {
                            range.push(
                                ParsedNode::Int {
                                    val: item.to_string().chars().collect()
                                }
                            );
                        });

                        return vec![
                            ParsedNode::List { items: range }
                        ];
                    },
                    _ => { }
                }
            }
        }

        return vec![];
    }
}
