mod util;
mod model;

use util::{Config, ActionType};
use model::AsciiVid;

fn main() {

    let config = Config::new();
    let ascii_vid = AsciiVid::new(config.width, config.height);

    match config.action {
        ActionType::Play => {
            ascii_vid.play(config.input).expect("Unable to play the ascii video");
        },
        ActionType::Render => {
            ascii_vid.render(config.input, config.output).expect("Unable to pre render the video");
        },
        ActionType::Resize => {
            ascii_vid.resize(config.input, config.output);
        },
        ActionType::Split => {
            ascii_vid.split(config.input, config.output, config.format);
        }
    };
}