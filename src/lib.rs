pub mod config;
pub mod rule;
pub mod step;
pub mod builder;

pub struct System {
    config: config::Config,
    
    generated_string : String,
    iteration : i8,
}

 impl System {
    pub fn new(confi: config::Config) -> System {
        let str = confi.axiom.to_string();
        System { config: confi, generated_string: str, iteration: 0 }
    }

    pub fn generate_string(&mut self,iterations: i8) -> String {
        for _i in 0..=iterations {
            self.generated_string = builder::generate_string(&self);
        }

        self.iteration = iterations;
        return self.generated_string.to_string();
    }
}