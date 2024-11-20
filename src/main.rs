use std::fs;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]  
enum Tokens {

    // keywords
    #[regex("fun")]
    FunDecl,
    #[regex("signal")]
    SignalDecl,
    #[regex("derive")]
    DeriveDecl,

    // variables
    #[regex(r##""(?:[^"\\]|\\.)*""##, |lex| lex.slice().to_string())]
    String(String),
    #[token(r"{")]
    OpenBlock,
    #[token(r"}")]
    CloseBlock,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token(";")]
    Semicolon,
    #[token("=")]
    Eq,
    // handle floats typed in at some point lol
    #[regex("[0123456789]+", |lex| lex.slice().parse().ok())]
    Number(f64),
    #[regex("[a-zA-Z]+", |lex| lex.slice().to_string())]
    Symbol(String),
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[regex(r"[ \r\t\n\f]+")]
    Whitespace

}

fn main() {
    let x = fs::read_to_string("./example.kob").expect("failed to read file");

    println!("{x}");

    let mut lex = Tokens::lexer(&x);

    for result in lex {
        match result {
            Ok(token) => {
                match token {
                    Tokens::Whitespace => print!(" "),
                    Tokens::OpenBlock => print!("{{\n"),
                    Tokens::CloseBlock => print!("\n}}"),
                    Tokens::OpenParen => print!("("),
                    Tokens::CloseParen => print!(")"),
                    Tokens::Semicolon => print!(";\n"),
                    Tokens::Eq => print!("="),
                    a => print!("{:?}", a)
                }
                
            },
            Err(e) => println!("some error occurred: {:#?}", e),
        }
    }
}

enum Instr {
    Signal(),
    Derive(),
    Bind(), // variable binding
    FunDecl(),
    FunCall(),
}