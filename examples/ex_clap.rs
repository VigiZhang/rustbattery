#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    /* 1. advanced but verbose
    let matches = App::new("My Super Program")
                    .version("1.0")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .about("Does awesome things")
                    .arg(Arg::with_name("config")
                         .short("c")
                         .long("config")
                         .value_name("FILE")
                         .help("Sets a custom config file")
                         .takes_value(true))
                    .arg(Arg::with_name("INPUT")
                         .help("Sets the input file to use")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("v")
                         .short("v")
                         .multiple(true)
                         .help("Sets the level of verbosity"))
                    .subcommand(SubCommand::with_name("test")
                                .about("controls testing features")
                                .version("1.3")
                                .author("Someone E. <someone_else@other.com>")
                                .arg(Arg::with_name("debug")
                                     .short("d")
                                     .help("print debug information verbosity")))
                    .get_matches();
    */

    /* 2. simple, no advanced feature
    let matches = App::new("myapp")
                    .version("1.0")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .about("Does awesome things")
                    .args_from_usage(
                        "-c, --config=[FILE] 'Sets a custom config file'
                        <INPUT>              'Sets the input file to use'
                        -v...                'Sets the level of verbosity'")
                    .subcommand(SubCommand::with_name("test")
                                .about("controls testing features")
                                .version("1.3")
                                .author("Someone E. <someone_else@other.com>")
                                .arg_from_usage("-d, --debug 'Print debug information'"))
                    .get_matches();
    */

    /* 3. from yaml
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    */

    // 4. use macro
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (about: "Does awesome things")
        (@arg FILE: -c --config +takes_value "Sets a custom config file")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg verbose: -v --verbose "Print test information verbosely")
        (@subcommand test =>
            (about: "controls testing features")
            (version: "1.3")
            (author: "Someone E. <someone_else@gmail.com>")
            (@arg debug: -d ... "Sets the level of debugging information")
        )
    ).get_matches();

    // get argument or default
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // required argument
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // repeatable argument
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // subcommand
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }
}
