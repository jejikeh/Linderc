use rand::Rng;
use crate::System;

fn generate_next_string(c : char, system : &System) -> String {
    let mut rng = rand::thread_rng();
    for i in 0..system.config.rules.len() {
        if system.config.rules[i].rule.0 == c {
            return system.config.rules[i].rule.1[rng.gen_range(0..system.config.rules[i].rule.1.len())].to_string();
        }
    }
    
    return String::from(c);
}

pub fn generate_string(system : &System) -> String {
    let mut generated_string = String::new();

    for i in 0..system.generated_string.len() {
        generated_string.push_str(&generate_next_string(system.generated_string.chars().nth(i).unwrap(), system));
    }

    return generated_string;
}