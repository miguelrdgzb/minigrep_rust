use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let filename: String = args[1].clone();
        let query: String = args[2].clone();
        return Config { filename, query }
    }
}

pub fn run(config: Config){
    let content: String = fs::read_to_string(config.filename).expect("No se pudo abrir el archivo");
    let found: Vec<&str> = search(&config.query, &content);

    for line in found {
        println!("{}", line)
    }
}

fn search<'a> (query: &str, content: &'a str) -> Vec<&'a str>{

    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query){
            result.push(line)

        }
    }
    return result
}