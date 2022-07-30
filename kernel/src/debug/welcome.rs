use crate::{kprintln, scoped_color_change, debug::color::Color};

pub const WELCOME_MESSAGE: &'static str = "$$$$$$\\  $$\\                                \n$$  __$$\\ $$ |                              \n$$ /  \\__|$$ | $$$$$$\\   $$$$$$$\\ $$\\   $$\\ \n$$ |      $$ |$$  __$$\\ $$  _____|\\$$\\ $$  |\n$$ |      $$ |$$$$$$$$ |$$ /       \\$$$$  / \n$$ |  $$\\ $$ |$$   ____|$$ |       $$  $$<  \n\\$$$$$$  |$$ |\\$$$$$$$\\ \\$$$$$$$\\ $$  /\\$$\\ \n \\______/ \\__| \\_______| \\_______|\\__/  \\__|";
pub const IS_LAUNCHING: &'static str = "_       _                        _     _                    \n(_)     | |                      | |   (_)                  \n _ ___  | | __ _ _   _ _ __   ___| |__  _ _ __   __ _       \n| / __| | |/ _` | | | | '_ \\ / __| '_ \\| | '_ \\ / _` |      \n| \\__ \\ | | (_| | |_| | | | | (__| | | | | | | | (_| |_ _ _ \n|_|___/ |_|\\__,_|\\__,_|_| |_|\\___|_| |_|_|_| |_|\\__, (_|_|_)\n                                                 __/ |      \n                                                |___/       ";
pub const IS_READY: &'static str = " _                          _          __                              \n(_)                        | |        / _|                              \n _ ___   _ __ ___  __ _  __| |_   _  | |_ ___  _ __   _   _  ___  _   _ \n| / __| | '__/ _ \\/ _` |/ _` | | | | |  _/ _ \\| '__| | | | |/ _ \\| | | |\n| \\__ \\ | | |  __/ (_| | (_| | |_| | | || (_) | |    | |_| | (_) | |_| |\n|_|___/ |_|  \\___|\\__,_|\\__,_|\\__, | |_| \\___/|_|     \\__, |\\___/ \\__,_|\n                               __/ |                   __/ |            \n                              |___/                   |___/             \n";

pub enum Stage {
    Launching,
    Ready,
}

pub fn welcome(stage: Stage) {
    let (second_message, color) = match stage {
        Stage::Launching => (IS_LAUNCHING, Color::LightBlue),
        Stage::Ready => (IS_READY, Color::Pink),
    };
    scoped_color_change! {
        background: Color::Black;
        foreground: Color::Purple;
        body: {
            kprintln!("{}", WELCOME_MESSAGE);
        }
    }

    scoped_color_change! {
        background: Color::Black;
        foreground: color;
        body: {
            kprintln!("{}", second_message);
        }
    }
}