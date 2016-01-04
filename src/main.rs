/// # Make-A-Lisp interpreter

extern crate rl_sys; // GNU readline bindings
extern crate regex;

use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;



/// ## Types
#[derive(Debug, PartialEq, Clone)]
enum Mal {
    List(Vec<Mal>),
    Sym(String),
    Str(String),
    Num(i32),
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    L, //left paren
    R, //right paren
    Sym(String),
    Str(String),
    Num(i32),
}



///
/// ## Read
///



/// ### lex
/// Takes a string and returns tokens
fn lex(input: &str ) -> Vec<Token>{
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"#)
        .unwrap();

    let mut output = Vec::new();
    for cap in re.captures_iter(input) {
        if let Some(s) = cap.at(1) {
            if let Some(t) = tokenize(s) {
                output.push(t);
            }
        }
    }
    output
}

/// ### tokenize
///
/// Takes a string and maybe returns a token
fn tokenize(input: &str) -> Option<Token>{
    use Token::*;

    if input.len() == 0 {
        return None;
    }


    match input.chars().nth(0).unwrap() {
        ';'       => None,
        '('       => Some(L),
        ')'       => Some(R),
        '0'...'9' => Some(Num(i32::from_str(input).unwrap())),
        '"'       => Some(Str(input
                         .trim_matches('"')
                         .to_string())),
        _         => Some(Sym(input.to_string())),
    }
}

/// ## Parse errors
#[derive(Debug, PartialEq)]
enum ParseErr {
    Empty,
    Unmatched(Token),
    Unexpected(Token),
    Unknown,
}

/// ##Scanner
/// A representation of a state machine for purely functional parsing
#[derive (Debug, Clone, Copy)]
struct Scanner<'a> {
    vec: &'a Vec<Token>,
    pos: usize,
}


impl<'a> Scanner<'a> {
    //! This scanner is only syntactically object oriented...

    fn new(vec: &'a Vec<Token>) -> Scanner<'a> {
        Scanner{vec: vec, pos: 0}
    }

    /// Returns the current token.
    fn peek(self) -> Option<Token> {
        if self.pos < self.vec.len() {
            Some(self.vec[self.pos].clone())
        }else {
            None
        }
    }

    /// Returns next machine and the current token
    fn next(self) -> (Scanner<'a>, Option<Token>) {
        if self.pos < self.vec.len() {
            (
                Scanner{vec: self.vec, pos: self.pos + 1},
                Some(self.vec[self.pos].clone()),
            )
        }else {
            let new_scanner = self;
            (new_scanner, None)
        }
    }
}

fn parse(tokens: &Vec<Token>) -> Result<Mal, ParseErr> {
    let scanner = Scanner::new(tokens);

    match parse_form(scanner) {
        Ok((_, m)) =>  Ok(m),
        Err(e) => Err(e)
    }
}

fn parse_form (scanner: Scanner) -> Result<(Scanner, Mal), ParseErr> {
    use Token::*;

    let scanner = scanner;
    let val = scanner.peek();

    match val {
        Some(t) =>  {
            match t {
                L => parse_list(scanner),
                Str(s) => Ok((scanner.next().0, Mal::Str(s))),
                Sym(s) => Ok((scanner.next().0, Mal::Sym(s))),
                Num(n) => Ok((scanner.next().0, Mal::Num(n))),
                _ => Err(ParseErr::Unexpected(t))
            }
        },
        None => Err(ParseErr::Empty)
    }
}

fn parse_list (_scanner: Scanner) -> Result<(Scanner, Mal), ParseErr> {
    use Token::*;
    assert_eq!(_scanner.peek().unwrap(), L);

    let mut scanner  = _scanner.next().0;
    let mut vec: Vec<Mal> = Vec::new();

    while let Some(t) = scanner.peek() {
        match t {
            R => {
                return Ok((scanner.next().0, Mal::List(vec)));
            },
            _ => {
                match parse_form(scanner) {
                    Ok((new_scanner, node)) => {
                        scanner.pos = new_scanner.pos;
                        vec.push(node);
                    },
                    Err(e) => return Err(e),
                };
            }
        };
    }
    Err(ParseErr::Unmatched(R))
}


/// ### read
/// 
/// Reads a string and returns a valid AST.
fn read(input: &str) -> Result<Mal, ParseErr>{
    let tokens = lex(input);
    parse(&tokens)
}

///
/// # Eval
///
/// TODO: implement!

/// ## Eval errors
#[derive(Debug, PartialEq)]
enum EvalErr{
    InvalidList,
    ArityMismatch(usize),
    TypeMismatch,
    UndefinedSymbol(String),
    Unknown,
}

#[derive(Debug, PartialEq)]
enum Arity {
    Variadic,
    Nary(usize)
}

/// Function Container
struct Func {
    name: String,
    arity: Arity,
    function: Box<Fn(&[Mal]) -> Result<Mal, EvalErr>>,
}

impl Func {
    /// Constructor
    fn new(
        name: &str,
        arity: Arity,
        function: Box<Fn (&[Mal]) -> Result<Mal, EvalErr>>)
            -> Func {
        Func {
            name: name.to_string(),
            arity: arity,
            function: function,
        }
    }

    fn apply(&self, args: &Vec<Mal>) -> Result<Mal, EvalErr>{
        use Arity::*;
        use EvalErr::*;

        // println!("{}, arity: {:?}", self.name, self.arity);
        // println!("args: {:?}", args);

        if let Nary(n) = self.arity {
            if args.len() - 1 != n {
                return Err(ArityMismatch(n));
            }
        }

        let f = &self.function;

        let mut evaled_args: Vec<Mal> = Vec::new();
        for x in &args[1..] {
            match eval(x) {
                Ok(x) => {
                    evaled_args.push(x);
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        f(&evaled_args)
    }
}


struct Scope {
    table: HashMap<String, Func>
}

impl Scope{
    fn new() -> Scope{
        Scope {
            table: HashMap::new()
        }
    }
    fn add_func(&mut self, func: Func) {
        self.table.insert(func.name.to_string(), func);
    }
    fn get_func(&self, sym: &str) -> Option<&Func>{
        self.table.get(sym)
    }
}


fn eval(input: &Mal) -> Result<Mal, EvalErr>{
    use Mal::*;
    use Arity::*;
    use EvalErr::*;


    let mut global = Scope::new();

    //increment
    global.add_func(Func::new("+1", Nary(1), Box::new(move |list: &[Mal]| -> Result<Mal, EvalErr> {
        match list[0] {
            Num(i) => Ok(Num(i + 1)),
            _ => Err(TypeMismatch)
        }
    })));

    // sum
    global.add_func(Func::new("+", Variadic, Box::new(move |list: &[Mal]| -> Result<Mal, EvalErr> {
        // TODO: implement with rust iterator functions: map, fold, collect, etc.
        let mut sum: i32 = 0;

        for m in list {
            if let Num(n) = *m {
                sum += n;
            } else {
                return Err(TypeMismatch);
            }
        }

        Ok(Num(sum))
    })));

    let global = &global;

    match *input {
        List(ref list) => {
            if list.len() > 0 {
                let first = &list[0];
                match *first {
                    Sym(ref sym) => {
                        // println!("{:?}", sym);
                        let func = global.get_func(sym);
                        match func {
                            Some(ref func) => func.apply(&list),
                            None => Err(UndefinedSymbol(sym.to_string()))
                        }
                    },
                    _ => Err(InvalidList)
                }


            } else {
                Err(InvalidList)
            }
        },
        //TODO: Optimization: can we not clone the input?
        _ => Ok(input.clone())
    }
}





///
/// # Print
///

fn print(ast: &Mal) -> String {
    use Mal::*;
    match *ast {
        List(ref list) => print_list(list),
        Sym(ref s) => s.to_string(),
        Str(ref s) => s.to_string(),
        Num(n) => n.to_string(),
    }
}

fn print_list(list: &Vec<Mal>) -> String {
    let mut sum = "(".to_string();
    if list.len() > 0 {
        for x in list {
            sum = sum + &print(x);
            sum = sum + " ";
        }
        sum.pop();
    }
    sum + ")"
}



// Main
fn main() {
    loop {
        let input: String = match rl_sys::readline("user> ") {
            Ok(Some(s)) => s,
            Ok(None) => break, // Ctrl-D
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let ast = read(&input);

        match ast {
            Ok(ast) => {
                // println!("; {:?}", input);
                let output = eval(&ast);
                match output {
                    Ok(output) => {
                        println!("{}", print(&output));
                    },
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                };
            },
            Err(e) => {
                use ParseErr::*;
                match e {
                    Empty => {},
                    _ => println!("Error: {:?}", e),
                }
            }
        };

        // println!("{:?}", output);
        if let Err(e) = rl_sys::add_history(&input) {
            panic!(e);
        }
    }
}


/// # TESTS:


#[test]
fn test_tokenize() {
    use Token::*;

    // test invalid
    assert_eq!(tokenize(""), None);
    assert_eq!(tokenize(";comment"), None);


    // test valid
    assert_eq!(tokenize("(").unwrap(), L);
    assert_eq!(tokenize(")").unwrap(), R);
    assert_eq!(tokenize("123").unwrap(), Num(123));
    assert_eq!(tokenize("first").unwrap(), Sym("first".to_string()));
    assert_eq!(tokenize("\"hello\"").unwrap(), Str("hello".to_string()));
}

#[test]
fn test_lex() {
    use Token::*;

    assert_eq!(lex(""), []);
    assert_eq!(lex(";comment"), []);
    assert_eq!(lex("()"), [L, R]);
    assert_eq!(lex("( ) ;comment"), [L, R]);

    assert_eq!(
        lex("(+ 1 2 3 \"hello\" )"),
        [L, Sym("+".to_string()), Num(1), Num(2), Num(3), Str("hello".to_string()), R]
            );
}



#[allow(dead_code)]
enum ReadPrintTestCase<'a, 'b> {
    Success(&'a str, &'b str),
    Error(&'a str, ParseErr)
}

#[test]
fn test_read_print() {
    use Token::*;
    use ParseErr::*;
    use ReadPrintTestCase::*;

    let cases = [
        Error("", Empty),
        Error(";hohoho", Empty),
        Error(" )", Unexpected(R)),
        Error("(", Unmatched(R)),
        Error("(;adsf", Unmatched(R)),
        Success("()", "()"),
        Success(" (     ) ; hello", "()"),
        Success("(+ 1 2 3)", "(+ 1 2 3)"),
        Success("( + ( - 3 2 ) 1 )", "(+ (- 3 2) 1)"),
    ];

    for tup in &cases {
        match *tup {
            Error(ref val, ref error) =>
                assert_eq!(read(val).err().unwrap(), *error),
            Success(ref val, ref expected) => 
                assert_eq!(print(&read(val).unwrap()), *expected),
        }
    }
}
