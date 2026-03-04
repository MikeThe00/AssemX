use std::env;
use std::process;
use std::fs;
use std::io;
use std::fmt;
use scanlex::{Scanner, Token};
//use std::io::{self, BufRead, BufReader};

// walk-tree-interpreter in rust from java code in the book craftinginterpreters

fn main() {
    enum Expr{
     Atom(i64),
     Oper(char,Vec<Expr>)
    }

    impl Expr{
        fn eval(&self) -> f32{
            match self{
                Expr::Atom(c) => {
                 match c {
                     i64::MIN..=i64::MAX => {
                        let c_f32: f32 = c.clone() as f32;
                        return c_f32;
                     },
                     _ => todo!(),
                 }
                },
                Expr::Oper(op, opands) => {
                let lhs = opands.first().unwrap().eval();
                let rhs = opands.last().unwrap().eval();
                match op {
                    '+' => return lhs + rhs,
                    '-' => return lhs - rhs,
                    '*' => return lhs * rhs,
                    '/' => return lhs / rhs,
                    _ => todo!(),
                    
                }
                },
                _ => todo!(),
            }
        }
    }

    impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Atom(i) => write!(f, "{}", i),
            Expr::Oper(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

    fn infix_binding_power(op: char) ->(f32, f32){
        match op{
            '+' | '-' => (1.0, 1.1),
            '*' | '/' => (2.0, 2.1),
            _ => panic!("unknown operator: {:?}", op)
            // more op's later like ()'s but this lexer is a litte more better then what this is for
        }
    }

fn parse_expr(scan: &mut Scanner, min_bp: f32) -> Expr{
   let mut lhs = match scan.get() {
          Token::Int(it) => Expr::Atom(it),
           Token::End => panic!("hit the end of source"),
           _ => todo!(),
           // this should be where the ()'s go recursively with 0.0 for binding power
    };
    //need to fix op to = to scan.peek() but it return's char's
    //so I have to use scan.get() because I can't find a way to use peek right
   loop {
    //you need a space so this runs right 
    let op = match scan.get() {
          Token::End => return lhs,
          Token::Char(op) => op,
          _ => todo!(),
     };
     //I need to put this back
    //scan.get();
    let (l_bp, r_bp) = infix_binding_power(op);
    if l_bp < min_bp{
        break;
    }
    let rhs = parse_expr(scan, r_bp); 
    lhs = Expr::Oper(op, vec![lhs, rhs]);
    }
   lhs
}
    // code here later

    fn run_files(path: &str) {
     let bytes: Vec<u8> = fs::read(path).expect("could not read file or there is no {path}");
     // should fix path with "/"s like Paths.get() in java
     let bytes_to_string = String::from_utf8(bytes);

     run(&bytes_to_string.expect("IDK"));
    }

    fn run_prompt(){
     let mut input = String::new();

     while input.trim() != "EXIT" {
     //will not exit while loop if you don't .trim(); for \n 
     //println!("debug: input = {input}");
     input.clear();
     io::stdin()
         .read_line(&mut input)
         .expect("failed to read line");
     run(&input.trim());
     }
     //debug
     println!("run_prompt worked");
    }

    fn run(source: &str) {
     //let mut tokens: Vec<Token> = Vec::new();
     let mut scan = Scanner::new(source);
     /*while let s = scan.get() {
         if s == Token::End {
          break
         }
         tokens.push(s);
     }*/
     //debug
     //println!("{:?}", tokens);
     let output = parse_expr(&mut scan, 0.0).eval();
     println!("{}", output);
     //code later 
     /*for i in 0..tokens.len() {
        let n = &tokens[i];  
        match n {
          Token::Num(_) => {println!("Num found"); /*code later*/},
          Token::Int(_) => {println!("Int found"); /*code later*/},
          Token::Str(_) => {println!("Str found"); /*code later*/},
          Token::Iden(_) => {println!("Iden found"); /*code later*/},
          Token::Char(_) => {println!("Char found"); /*code later*/},
          Token::Error(_) => {todo!(); /*code later*/},
          Token::End => {println!("EOF"); /*code later*/}
          //code later
        }
     }
     //for loop with enum and struct's
     */
    }

    let args: Vec<String> = env::args().collect();
    // this put each arg a string into a vector with a index like a array
    // arrays in rust are fixed. vectors in rust are not so we can use at runtime

    if args.len() > 2 {
      println!("Usage: AssemX [script]");
      process::exit(64);
    } else if args.len() == 2{
      run_files(&args[1]);
      // & / &str because borrow ship system and I don't want a .clone() on heap
      //use a reference we just need to read the string 
      //why not 0 because 0 is the path to executable/program
    } else if args.len() == 1{
      run_prompt();
    }
}
