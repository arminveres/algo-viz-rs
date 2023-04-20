mod sorting;

use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

use rand::{self, Rng};

const RESOLUTION: (f32, f32) = (400.0, 400.0);
const NO_RECTS: usize = 150;

/// Keeps track of current variables and states
struct GameState {
    frames: usize,
    objects: Vec<graphics::Mesh>,
    rectangles: [graphics::Rect; NO_RECTS],
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        // TODO: (aver) implement scaling for window and ajdust for rendering
        const SCALE: f32 = 0.75;
        let mut rng = rand::thread_rng();
        let mut l_rectangles: [graphics::Rect; NO_RECTS] = [graphics::Rect::default(); NO_RECTS];

        for i in 0..l_rectangles.len() {
            l_rectangles[i].w = 10.;
            l_rectangles[i].h = -rng.gen_range(0.0..750.0);
            l_rectangles[i].y = 1000.;
            l_rectangles[i].x += 100. + i as f32 * l_rectangles[i].w * SCALE;
            // println!("{}, {}", l_rectangles[i].x, i);
        }

        let mut l_objects = vec![];

        for i in 0..NO_RECTS {
            l_objects.push(graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                l_rectangles[i],
                Color::WHITE,
            )?);
        }

        Ok(GameState {
            frames: 0,
            objects: l_objects,
            rectangles: l_rectangles,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // TODO: implement update logic
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // NOTE: Drawing starts from top left!

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let mut i = 0;
        for obj in &self.objects {
            canvas.draw(obj, Vec2::new(self.rectangles[i].x, RESOLUTION.1));
            i += 1;
        }

        canvas.finish(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps());
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("algo-vis-rs", "Armin Veres").build()?;

    let state = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
