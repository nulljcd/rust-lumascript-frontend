pub mod error_handler;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod token;

fn run(input: &str, error_handler: &mut error_handler::ErrorHandler) -> Option<()> {
    let mut lexer: lexer::Lexer<'_> = lexer::Lexer::new(input);
    let mut parser: parser::Parser<'_> = parser::Parser::new(&mut lexer);

    let program_node: node::Node = parser.parse(error_handler)?;

    println!("{program_node:#?}");

    Some(())
}

pub fn main() {
    let mut error_handler: error_handler::ErrorHandler = error_handler::ErrorHandler::new();

    let input: &str = "
        fib := func n {
            if n < 2 {
                return n;
            } else {
                return fib(n - 1) + fib(n - 2);
            }
        };

        fib(24);
    ";

    if run(input, &mut error_handler).is_none() {
        error_handler.print_error();
    }
}
