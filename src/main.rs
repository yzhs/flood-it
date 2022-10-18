use macroquad::prelude::*;

use game::Game;

mod colour;
mod game;
mod graph;
mod grid;
mod ui;

/// Handle command line arguments
fn parse_args() -> (u32, u32) {
    use clap::{value_parser, Arg, Command};

    let matches = Command::new("Flood-It")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("colors")
                .value_name("colors")
                .help("The number of different colors")
                .value_parser(value_parser!(u32))
                .default_value("6"),
        )
        .arg(
            Arg::new("size")
                .value_name("size")
                .help("The height and width of the grid")
                .value_parser(value_parser!(u32))
                .default_value("14"),
        )
        .get_matches();

    let colors = {
        let tmp = *matches
            .get_one::<u32>("colors")
            .expect("Invalid number of colors") as usize;
        let maximum_number_of_colours = colour::ALL_COLOURS.len();
        if tmp < 3 || tmp > maximum_number_of_colours {
            panic!(
                "Flood-It only supports 3 through {} (inclusive) colors.",
                maximum_number_of_colours,
            );
        } else {
            tmp as u32
        }
    };

    let size: u32 = {
        let tmp = *matches.get_one::<u32>("size").expect("Invalid grid size");
        if tmp < 2 {
            panic!("Flood-It needs a grid of at least 2x2 cells.");
        } else {
            tmp
        }
    };

    (colors, size)
}

#[macroquad::main("Flood-It")]
async fn main() {
    let (number_of_colours, size) = parse_args();

    let mut game = Game::create(size, number_of_colours);
    let mut ui = crate::ui::Ui::create(size, number_of_colours);

    loop {
        if let Some(KeyCode::Q) = get_last_key_pressed() {
            break;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            ui.handle_click(&mut game, mouse_position());
        }

        if is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::N)
            || is_key_pressed(KeyCode::R)
        {
            ui.handle_key_press(&mut game);
        }

        ui.resize();
        ui.render(&game);

        next_frame().await
    }
}
