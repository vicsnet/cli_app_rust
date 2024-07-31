use std::env;
// use std::error::Error;
// read file
// use std::fs;
// exist the process without panicking
use std::process;

use cli_app_rust::Config;
//

fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let filename = &args[2];

    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Process parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    println!("{:?}", args);

    // let contents =fs::read_to_string(config.filename).expect("Something went wrong readingthe file");

    // println!("With text:\n{}", contents);

    if let Err(e) = cli_app_rust::run(config) {
        eprintln!("Application error:{}", e);
        process::exit(1);
    };
}

// fn run(config: Config){
//     let contents =fs::read_to_string(config.filename).expect("Something went wrong readingthe file");

//     println!("With text:\n{}", contents);
// }
// fn run(config: Config)-> Result<(), Box<dyn Error>>{
//     let contents =fs::read_to_string(config.filename)?;

//     println!("With text:\n{}", contents);
//     Ok(())
// }

// struct Config{
//     query: String,
//     filename:String
// }

// impl Config{
// //     fn new(args: &[String]) -> Config
// // {
// //     if args.len() < 3{
// //         panic!("not enough arguments");
// //     }
// //     let query = args[1].clone();
// //     let filename = args[2].clone();

// //     Config{query, filename}
// // }
//     fn new(args: &[String]) -> Result<Config, &str>
// {
//     if args.len() < 3{
//         return Err("not enough arguments");
//     }
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Ok(Config{query, filename})
// }
// }
// fn parse_config(args: &[String]) ->
// // (&str, &str)
// Config
// {
//     // let query = &args[1];
//     // let filename = &args[2];
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     // (query, filename)
//     Config{query, filename}
// }
