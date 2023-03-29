extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

static TEST: &'static str = include_str!("test.frl");

#[derive(Parser)]
#[grammar = "parser.pest"]
struct FRLParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        println!("Here");
        let out = FRLParser::parse(Rule::script, TEST);
        if out.is_err() {
            println!("{}", out.err().unwrap().to_string());
        } else {
            println!("{}", out.unwrap());
        }
    }
}