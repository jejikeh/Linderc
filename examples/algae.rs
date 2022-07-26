extern crate linde;
use linde::System;
use linde::rule::Rule;
use linde::config::Config;


fn main(){
    let rules = vec![
        Rule::new('A',vec![String::from("AB")]/* ,0.0,false,1.0 */),
        Rule::new('B',vec![String::from("A")]/* ,0.0,false,1.0 */),
    ];
    let config = Config::new(String::from("A"),rules);
    let mut system = System::new(config);

    println!("{}",system.generate_string(2));
}