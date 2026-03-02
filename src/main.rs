use std::env;
use std::process;
use std::fs;
use std::io;
use scanlex::{Scanner, Token};
//use std::io::{self, BufRead, BufReader};

// walk-tree-interpreter in rust from java code in the book craftinginterpreters

fn main() {
    enum Ast_tree {
        Binary_expr(Ast_binary),
        Decl(Ast_decl)
        //Statements
        //Expressions
        //Literals
        //Identifiers/Names
    }
    
    //struct Ast_tree

    //struct Ast1_module {
     //module: String,
     //in_scope: bool,
     //string_of_bytes_to_run: String
     //code later 
     //this will run other modules (files) to use fn's or other things later
    //}

    struct Ast_decl {
        is_var: bool,
        is_module: bool
        //code later for fun's 
    }

    struct Ast_binary {
        left: String,
        op: char,
        right: i32
    }

    impl Ast_decl {
       fn iden_find(token: &Token) -> Ast_decl{
         if token == &Token::Iden("module".to_string()){
         return Ast_decl{
           is_module: true,
           is_var: false
          }
         }
         //else if code later
        return Ast_decl{
         is_module: false,
         is_var: false
        }
       }
    }

    fn run_files(path: &str) {
     let bytes: Vec<u8> = fs::read(path).expect("could not read file or there is no {path}");
     // should fix path with "/"s like Paths.get() in java
     let bytes_to_string = String::from_utf8(bytes);

     run(&bytes_to_string.expect("IDK"));
     // debug
     println!("run_files worked");
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
     let mut tokens: Vec<Token> = Vec::new();
     let mut scan = Scanner::new(source);
     while let s = scan.get() {
         if s == Token::End {
          break
         }
         tokens.push(s);
     }
     //debug
     println!("{:?}", tokens);
     //code later
     for i in 0..tokens.len() {
        let n = &tokens[i];  
        match n {
          Token::Iden(_) => {
              //if tokens[i] == Token::Iden("module".to_string()){
              //debug
               println!("Iden found");
               let find = Ast_decl::iden_find(n);
               //Ast_tree::Decl(find);
               let found: bool = find.is_module;
               println!("{}", find.is_module);
              //code later
          },
          _ => todo!()
          //code later
        }
     }
     //for loop with enum and struct's
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
