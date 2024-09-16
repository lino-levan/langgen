use tokenize::tokenize;

mod config;
mod tokenize;

fn main() {
    let conf = config::Config {
        variable_declaration: "let".to_string(),
        variable_initialize: "=".to_string(),
        line_end: ";".to_string()
    };
    let tokens = tokenize(conf, "let x = 0;".to_string());
    println!("{:?}", tokens);
    let conf2 = config::Config {
        variable_declaration: "define".to_string(),
        variable_initialize: "as".to_string(),
        line_end: ".".to_string()
    };
    let tokens2 = tokenize(conf2, "define x as 0.".to_string());
    println!("{:?}", tokens2);
}
