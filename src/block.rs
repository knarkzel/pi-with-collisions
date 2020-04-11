use ggez::{graphics, Context, GameResult};
use mint::Point2;

#[derive(Copy, Clone)]
pub struct Block {
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub mass: f64,
}

impl Block {
    pub fn new(x: f64, y: f64, speed: f64, mass: f64) -> Block {
        Block { x, y, speed, mass }
    }
    pub fn update(&mut self) -> bool {
        if self.x < 0.0 {
            self.x = 0.0;
            self.speed = -self.speed;
            return true;
        }
        self.x += self.speed;
        false
    }
    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(0.0, 0.0, 64.0, 64.0);
        let rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::BLACK)?;
        graphics::draw(
            ctx,
            &rect,
            graphics::DrawParam::default().dest(Point2 {
                x: self.x as f32,
                y: self.y as f32,
            }),
        )?;
        Ok(())
    }
}
