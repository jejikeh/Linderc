use crate::rule::Rule;

#[derive(Clone)]
pub struct Config {
    pub axiom : String,
    pub rules : Vec<Rule>,
}

impl Config {
    pub fn new(axiom : String,rules: Vec<Rule>) -> Self {
        Config { axiom: (axiom), rules: (rules) }
    }
}