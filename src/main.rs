use game_prototype::block::*;
use ggez::event::{self, EventHandler};
use ggez::{audio, audio::SoundSource, input, input::keyboard::KeyCode, graphics, Context, ContextBuilder, GameResult};
use std::{convert::From, env, path::PathBuf};

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        PathBuf::from("./resources")
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("Title", "Game Author")
        .add_resource_path(resource_dir)
        .build()
        .expect("Unable to build context.");

    let mut my_game = Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Game {
    blocks: Vec<Block>,
    digits: f64,
    timesteps: f64,
    collisions: u64,
    clack: audio::Source,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        let digits = 1.0;
        let timesteps = 1.0;
        Game {
            blocks: vec![
                Block::new(200.0, 250.0, 0.0, 1.0),
                Block::new(400.0, 250.0, -1_f64 / timesteps, 100_f64.powf(digits - 1.0)),
            ],
            digits,
            timesteps,
            collisions: 0,
            clack: audio::Source::new(ctx, "/clack.wav").unwrap(),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if input::keyboard::is_key_pressed(ctx, KeyCode::R) {
            self.blocks[0] = Block::new(200.0, 250.0, 0.0, 1.0);
            self.blocks[1] = Block::new(400.0, 250.0, -1_f64 / self.timesteps, 100_f64.powf(self.digits - 1.0));
            self.collisions = 0;
        }
        let mut play = false;
        for _ in 0..self.timesteps as u64 {
            if do_collide(&self.blocks[0], &self.blocks[1]) {
                let m1 = self.blocks[0].mass;
                let m2 = self.blocks[1].mass;
                let v1 = self.blocks[0].speed;
                let v2 = self.blocks[1].speed;
                let newv1 = ((m1 - m2) / (m1 + m2)) * v1 + ((2.0 * m2) / (m1 + m2)) * v2;
                let newv2 = ((2.0 * m1) / (m1 + m2)) * v1 + ((m2 - m1) / (m1 + m2)) * v2;
                self.blocks[0].speed = newv1;
                self.blocks[1].speed = newv2;
                self.collisions += 1;
                play = true;
            }
            for block in self.blocks.iter_mut() {
                if block.update() {
                    play = true;
                    self.collisions += 1;
                }
            }
        }
        if play {
            self.clack.play()?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        for block in self.blocks.iter() {
            block.render(ctx)?;
        }
        let mut text = graphics::Text::new(self.collisions.to_string());
        text.set_font(graphics::Font::default(), graphics::Scale::uniform(100.0));
        graphics::draw(
            ctx,
            &text,
            graphics::DrawParam::default().color(graphics::BLACK),
        )?;
        graphics::present(ctx)
    }
}

fn do_collide(block1: &Block, block2: &Block) -> bool {
    if block1.x + block1.speed < block2.x + 64.0 && block1.x + 64.0 + block1.speed > block2.x {
        return true;
    }
    false
}
