use ggez::{event, glam::*, graphics, Context, GameResult};
use sorting::{BubbleSort, Sorter};

// TODO: (aver) implement dynamic screen scaling, as in the snake example
const DESIRED_FPS: u32 = 10;
const RESOLUTION: (u32, u32) = (2250, 900);

/// Keeps track of current variables and states
struct GameState<T: Sorter> {
    frames: usize,
    // TODO: make dynamic, ERROR: result not fixed at compile time...
    sorter: T,
}

impl<T: Sorter> GameState<T> {
    fn new(sorter: T, ctx: &mut Context) -> GameResult<GameState<T>> {
        // TODO: (aver) implement scaling for window and ajdust for rendering
        Ok(GameState {
            frames: 0,
            sorter: sorter,
        })
    }
}

impl<T: Sorter> event::EventHandler<ggez::GameError> for GameState<T> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // TODO: implement update logic

        while _ctx.time.check_update_time(DESIRED_FPS) {
            if !self.sorter.is_sorted() {
                self.sorter.step(_ctx);
            } else {
                // println!("it's sorted!");
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // NOTE: Drawing starts from top left!

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let mut i = 0;
        let arr = self.sorter.get_arr();

        for obj in arr {
            canvas.draw(&obj.mesh, Vec2::new(arr[i].rect.x, RESOLUTION.1 as f32));
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
    // TODO: (aver) add argparsing
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("algo-vis-rs", "Armin Veres")
        .window_setup(ggez::conf::WindowSetup::default().title("Sorting Algorithm Visualizer"))
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(RESOLUTION.0 as f32, RESOLUTION.1 as f32),
        )
        .build()?;

    let l_sorter = BubbleSort::new(&mut ctx);
    let state = GameState::new(l_sorter, &mut ctx)?;

    event::run(ctx, event_loop, state)
}
