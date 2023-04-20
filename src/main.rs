mod sorting;

use ggez::{
    event,
    glam::*,
    graphics::{self, Color, Rect},
    Context, GameResult,
};

/// Keeps track of current variables and states
struct MainState {
    pos_x: f32,
    frames: usize,
    object: graphics::Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let rect = Rect {
            x: 0.,
            y: 0.,
            h: 200.,
            w: 50.,
        };
        let rectangle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::WHITE)?;

        Ok(MainState {
            pos_x: 10.0,
            frames: 0,
            object: rectangle,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // self.pos_x = self.pos_x % 800.0 + 1.0;
        self.pos_x = self.pos_x % 3440. + 10.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // NOTE: Drawing starts from top left!
        canvas.draw(&self.object, Vec2::new(self.pos_x, 1440.0 - 200.));

        canvas.finish(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps());
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("algo-viz-rs", "Armin Veres").build()?;

    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
