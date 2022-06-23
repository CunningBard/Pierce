use crate::lexer::{Lexer, Token, TokenType};
use crate::ast_base::YAPL;

pub struct Parser {
    ind: i32,
    run: bool,
    lexer: Lexer,
    current_token: Token,
    to_parse: Vec<Token>,
    defined_names: Vec<String>,
    whole_program: Vec<YAPL>
}

impl Parser {
    pub fn new() -> Self{
        Self {
            ind: -1,
            run: true,
            lexer: Lexer::new(),
            current_token: Token::new(TokenType::NullForParser, "".to_string()),
            to_parse: vec![],
            defined_names: vec![],
            whole_program: vec![]
        }
    }
    fn next_token(&mut self) -> bool {
        self.ind += 1;
        if self.ind >= self.to_parse.len() as i32{
            false
        } else {
            self.current_token = self.to_parse[self.ind as usize].clone();
            true
        }
    }
    pub fn parse_text(&mut self, text: String){
        self.to_parse = self.lexer.lex_text(text);
        self.parse()
    }
    fn parse(&mut self){

        while self.run {
            if !self.next_token() {
                self.run = false;
                break
            }
            if self.current_token.token_type == TokenType::Let {
                if !self.next_token() || self.current_token.token_type != TokenType::Identifier {
                    panic!("Error expected a variable name got '{:?}' , at char {} line {}", self.current_token.token_type, self.current_token.x, self.current_token.y)
                }
                let var_name = self.current_token.clone();

                if !self.next_token() || self.current_token.token_type != TokenType::Arrow {
                    panic!("Error expected a type identifier symbol got '{:?}' , at char {} line {}", self.current_token.token_type, self.current_token.x, self.current_token.y)
                }

                if !self.next_token() || self.current_token.token_type != TokenType::Identifier {
                    panic!("Error expected a variable type got '{:?}' , at char {} line {}", self.current_token.token_type, self.current_token.x, self.current_token.y)
                }
                let var_type = self.current_token.clone();

                if !self.next_token() || self.current_token.token_type != TokenType::AssignmentOperator {
                    panic!("Error expected a Assignment Operator got '{:?}' , at char {} line {}", self.current_token.token_type, self.current_token.x, self.current_token.y)
                }

                let mut values: Vec<Token> = vec![];
                loop {
                    self.next_token();

                    if self.current_token.token_type == TokenType::EndOfFile{
                        panic!("Error! expected End of line got End of File instead")
                    }
                    if self.current_token.token_type == TokenType::EndLine {
                        break
                    } else if self.current_token.is_data_type() || self.current_token.token_type == TokenType::MathOperation {
                        values.push(self.current_token.clone());
                    }
                }
                print!("{}: {} = ", var_name.value, var_type.value);

            }
            else if self.current_token.token_type == TokenType::Identifier {
                if !self.next_token(){
                   panic!("Expected operator at line {} char {}", self.current_token.y, self.current_token.y)
                }
                if self.current_token.token_type == TokenType::ParenthesisOpen {
                    let mut func_args = vec![];
                    let mut expect_comma = false;
                    while self.current_token.token_type != TokenType::ParenthesisClose {
                        if !self.next_token(){
                           panic!("Expected arguments got '{:?}' instead, at line {} char {}", self.current_token.token_type, self.current_token.y, self.current_token.x)
                        }
                        if self.current_token.token_type == TokenType::SeperatorComma {
                            if !expect_comma {
                               panic!("Expected arguments got '{:?}' instead, at line {} char {}", self.current_token.token_type, self.current_token.y, self.current_token.x)
                            }
                            expect_comma = false;
                        } else if self.current_token.token_type == TokenType::Identifier {
                            if expect_comma {
                               panic!("Expected separator got '{:?}' instead, at line {} char {}", self.current_token.token_type, self.current_token.y, self.current_token.x)
                            }

                            func_args.push(self.current_token.clone());
                            expect_comma = true;
                        } else if self.current_token.token_type == TokenType::ParenthesisClose {
                            break
                        } else {
                            panic!("dude, come on '{:?}' , at line {} char {}", self.current_token.token_type, self.current_token.y, self.current_token.x);
                        }
                    }

                    if !self.next_token() && self.current_token.token_type != TokenType::EndLine {
                        panic!("Expected end of line got '{:?}' instead, at line {} char {}", self.current_token.token_type, self.current_token.y, self.current_token.x)
                    }
                    println!("{:?}", func_args);
                }
            }
        }
    }
}