/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/
extern crate ggez;

use constants::{SQUARE_SIZE, MID_ROW, NUM_ROW, NUM_LANE, NUM_LOG, ROAD, RIVER, WIN_W, WIN_H};
use sprites::Rectangle;
use ggez::{GameResult, Context};
use ggez::graphics::{self};


pub struct Road {
    form: Rectangle
}

pub struct River {
    form: Rectangle
}

pub struct Menu {
}

// pub struct Cubbie {
//     form: Rectangle
// }


impl Road {
    pub fn new(w: u32, h: u32) -> Road {
        Road {
            form: Rectangle::construct(
                0.0,
                h as f32 - (MID_ROW - 1) as f32 * SQUARE_SIZE,
                w as f32,
                NUM_LANE as f32 * SQUARE_SIZE,
                ROAD,
            ),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }
}

impl River {
    pub fn new(w: u32, h: u32) -> River {
        River {
            form: Rectangle::construct(
                0.0,
                h as f32 - (NUM_ROW as f32 - 3.0) * SQUARE_SIZE,
                w as f32,
                NUM_LOG as f32 * SQUARE_SIZE + 1.0,
                RIVER,
            ),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }
}

impl Menu {
    pub fn draw(&mut self, ctx: &mut Context, selection: u32) -> GameResult<()> {
        //Draw Crabber name upper-middle
        let game_name = format! {"CRABBER"};
        let font = graphics::Font::new(ctx, "/game_over.ttf", 56).unwrap();
        let name_text = graphics::Text::new(ctx, &game_name, &font)?;
        let horizontal: f32 = WIN_W as f32 / 2.0 - name_text.width() as f32 / 2.0;
        let vertical: f32 = WIN_H as f32 / 2.0 + SQUARE_SIZE * 1.0;
        let dest_point = graphics::Point2::new(horizontal, vertical);
        graphics::draw(ctx, &name_text, dest_point, 0.0)?;

        //Draw the Crab logo
        let image_big_crab = graphics::Image::new(ctx, "/crab.png")?;
        let horizontal_crab: f32 = WIN_W as f32 / 2.0 - image_big_crab.width() as f32 / 2.0;
        let vertical_crab: f32 = vertical - SQUARE_SIZE * 8.0;
        let dest_point = graphics::Point2::new(horizontal_crab, vertical_crab);
        graphics::draw(ctx, &image_big_crab, dest_point, 0.0)?;

        //Draw Start option
        let start = format! {"Start"};
        let font_start = graphics::Font::new(ctx, "/game_over.ttf", 20).unwrap();
        let start_text = graphics::Text::new(ctx, &start, &font_start)?;
        let horizontal2: f32 = WIN_W as f32 / 2.0 - start_text.width() as f32 / 2.0;
        let vertical2: f32 = WIN_H as f32 / 2.0 + SQUARE_SIZE * 5.0;
        let dest_point = graphics::Point2::new(horizontal2, vertical2);
        graphics::draw(ctx, &start_text, dest_point, 0.0)?;

        //Draw Scores option
        let scores = format! {"Scores"};
        let font_score = graphics::Font::new(ctx, "/game_over.ttf", 20).unwrap();
        let score_text = graphics::Text::new(ctx, &scores, &font_score)?;
        let horizontal3: f32 = WIN_W as f32 / 2.0 - score_text.width() as f32 / 2.0;
        let vertical3: f32 = vertical2 + SQUARE_SIZE;
        let dest_point = graphics::Point2::new(horizontal3, vertical3);
        graphics::draw(ctx, &score_text, dest_point, 0.0)?;

        //Figure out where the crab selector goes
        if selection == 0 {
            let image_small_crab = graphics::Image::new(ctx, "/tiny_crab.png")?;
            let horizontal_small_crab: f32 = horizontal2 - 30.0;
            let dest_point = graphics::Point2::new(horizontal_small_crab, vertical2 + 10.0);
            graphics::draw(ctx, &image_small_crab, dest_point, 0.0)?;
        } else if selection == 1 {
            let image_small_crab = graphics::Image::new(ctx, "/tiny_crab.png")?;
            let horizontal_small_crab: f32 = horizontal3 - 30.0;
            let dest_point = graphics::Point2::new(horizontal_small_crab, vertical3 + 10.0);
            graphics::draw(ctx, &image_small_crab, dest_point, 0.0)?;
        }

        Ok(())
    }
}

