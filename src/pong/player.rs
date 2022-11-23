use macroquad::{texture::Image, prelude::WHITE};

use super::{GAME_HEIGHT, GAME_WIDTH};

const PLAYER_MARGIN: u32 = 10;
const PLAYER_HEIGHT: u32 = 80;
const PLAYER_WIDTH: u32 = 20;
const MAX_Y_POS: u32 = GAME_HEIGHT as u32 - PLAYER_HEIGHT;

#[derive(PartialEq)]
pub enum Side {
    LeftSide,
    RightSide,
}

fn get_side_starting_x(side: &Side) -> u32 {
    match side {
        Side::LeftSide => PLAYER_MARGIN,
        Side::RightSide => GAME_WIDTH as u32 - PLAYER_WIDTH - PLAYER_MARGIN,
    }
}

// fn get_side_ending_x(side: &Side) -> u32 {
//     match side {
//         Side::left_side => PLAYER_MARGIN + PLAYER_WIDTH,
//         Side::right_side => GAME_WIDTH as u32 - PLAYER_MARGIN,
//     }
// }

pub(super) struct Player {
    pub y_pos: u32,
    pub side: Side,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            y_pos: 0,
            side: Side::LeftSide,
        }
    }
}

impl Player {
    pub(super) fn move_down(&mut self) {
        let res = self.y_pos.checked_add(1);
        if res.is_some() {
            self.y_pos = res.unwrap();
        }
    }

    pub(super) fn move_up(&mut self) {
        let res = self.y_pos.checked_sub(1);
        if res.is_some() {
            self.y_pos = res.unwrap();
        }
    }

    pub(super) fn draw(&self, img: &mut Image) {
        let starting_x = get_side_starting_x(&self.side);
        for y in self.y_pos..(self.y_pos + PLAYER_HEIGHT) {
            for x in starting_x..(starting_x + PLAYER_WIDTH) {
                img.set_pixel(x, y, WHITE);
            }
        }
    }
}