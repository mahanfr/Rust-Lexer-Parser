extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct NemetParser;

fn main() {
    let pairs = NemetParser::parse(Rule::program_file, 
            "static name @u32 :: 32 + 2;\nfunc m2() @cum {} \n"
        )
        .unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        for inner_pair in pair.into_inner() {
            println!("\tRule:    {:?}", inner_pair.as_rule());
            println!("\tSpan:    {:?}", inner_pair.as_span());
            println!("\tText:    {}", inner_pair.as_str());
        }
    }


}

