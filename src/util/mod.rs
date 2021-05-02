pub mod command;

use clap::{Arg, App, SubCommand, AppSettings};

pub enum ActionType{
    Play,
    Render,
    Resize,
    Split
}

pub struct Config{
    pub action: ActionType,
    pub width: u16,
    pub height: u16,
    pub input: String,
    pub output: String,
    pub format: String
}

impl Config{
    pub fn new()-> Self{
        let arg_matches = Config::read_arguments().get_matches();
        let action : ActionType;
        let input: String;
        let mut output: String = String::from("");
        let mut format: String = String::from("");
        let width: u16;
        let height: u16;

        height = arg_matches.value_of("height")
            .unwrap()
            .to_string()
            .parse::<u16>()
            .unwrap();

        width = arg_matches.value_of("width")
            .unwrap()
            .to_string()
            .parse::<u16>()
            .unwrap();

        match arg_matches.subcommand(){
            ("play", sub_m) =>{
                action = ActionType::Play;
                input = sub_m.unwrap().value_of("input").unwrap().to_string();
            },
            ("render", sub_m) => {
                action = ActionType::Render;
                input = sub_m.unwrap().value_of("input").unwrap().to_string();
                output = sub_m.unwrap().value_of("output").unwrap().to_string();
            },
            ("split", sub_m) => {
                action = ActionType::Split;
                input = sub_m.unwrap().value_of("input").unwrap().to_string();
                output = sub_m.unwrap().value_of("output").unwrap().to_string();
                format = sub_m.unwrap().value_of("format").unwrap_or("frame_%d.png").to_string();
            },
            ("resize", sub_m) => {
                action = ActionType::Resize;
                input = sub_m.unwrap().value_of("input").unwrap().to_string();
                output = sub_m.unwrap().value_of("output").unwrap().to_string();
            },
            _ =>{std::process::exit(1)}
        };

        Config{
            action,
            width,
            height,
            input,
            output,
            format
        }
    }

    fn read_arguments()->clap::App<'static, 'static>{
        App::new("AsciiVid")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("Just another Ascii video player")
            .settings(&[
                AppSettings::ColoredHelp,
                AppSettings::DisableHelpSubcommand,
                AppSettings::VersionlessSubcommands
            ])
            .arg(Arg::with_name("width")
                .short("w")
                .long("width")
                .required(true)
                .takes_value(true)
                .help("Set the with of the final video player"))
            .arg(Arg::with_name("height")
                .short("h")
                .long("height")
                .required(true)
                .takes_value(true)
                .help("Set the height of the final video player"))

            .subcommand(SubCommand::with_name("play")
                .about("Reproduces a pre rendered Ascii video")
                .arg(Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .required(true)
                    .takes_value(true)
                    .help("Set the folder where the ascii frames are stored"))
                .setting(AppSettings::ColoredHelp))

            .subcommand(SubCommand::with_name("render")
                .about("Render a set of images to Ascii frames")
                .arg(Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .takes_value(true)
                    .required(true)
                    .help("Set the output folder for the Ascii frames"))
                .arg(Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .takes_value(true)
                    .required(true)
                    .help("Set the folder of your images"))
                .setting(AppSettings::ColoredHelp))

            .subcommand(SubCommand::with_name("split")
                .about("Splits a video into images")
                .arg(Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .takes_value(true)
                    .required(true)
                    .help("Set your input video"))
                .arg(Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .takes_value(true)
                    .required(true)
                    .help("Set your output folder"))
                .arg(Arg::with_name("format")
                    .long("format")
                    .short("f")
                    .takes_value(true)
                    .help("Set the structure name for the output images"))
                .setting(AppSettings::ColoredHelp))

            .subcommand(SubCommand::with_name("resize")
                .about("Resizes a video")
                .arg(Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .required(true)
                    .takes_value(true)
                    .help("Input video"))
                .arg(Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .takes_value(true)
                    .help("Output video"))
                .setting(AppSettings::ColoredHelp))
    }
}