pub struct Rule {
    pub rule : (char, Vec<String>),
    // action : fn(step : Step),
    // angle : f32,
    // draw_steps : bool,
    // length : f32,
}

impl Rule {
    pub fn new(
        a : char, b : Vec<String>, 
        /*action : fn(step : Step), angle : f32, 
        draw_steps : bool, length : f32 */) -> Rule {
            Rule { rule : (a, b), /* action : action, angle : angle, draw_steps : draw_steps, length : length */}
        }
}