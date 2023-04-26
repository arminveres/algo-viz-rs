use ggez::glam::Vec2;
use ggez::graphics::{self, Rect};
use ggez::{event, Context, GameResult};
use sorting::{BubbleSort, Sorter, INIT_WINDOW_SIZE};

const DESIRED_FPS: u32 = 10;

struct WindowSettings {
    resize_projection: bool,
}

struct GameState<T: Sorter> {
    frames: usize,
    window_settings: WindowSettings,
    screen_coords: Rect,
    sorter: T,
}

impl<T: Sorter> GameState<T> {
    fn new(given_sorter: T, ctx: &mut Context) -> GameResult<GameState<T>> {
        let s = GameState {
            frames: 0,
            window_settings: WindowSettings {
                resize_projection: false,
            },
            screen_coords: Rect {
                x: 0.,
                y: 0.,
                w: ctx.gfx.drawable_size().0,
                h: ctx.gfx.drawable_size().1,
            },
            sorter: given_sorter,
        };
        Ok(s)
    }
}

impl<T: Sorter> event::EventHandler<ggez::GameError> for GameState<T> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
            if !self.sorter.is_sorted() {
                self.sorter.step(ctx);
            } else {
                // println!("it's sorted!");
            }
        }
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // NOTE: Drawing starts from top left!
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        canvas.set_screen_coordinates(self.screen_coords); // set custom canvas for resizing

        let mut i = 0;
        let arr = self.sorter.get_arr();
        for obj in arr {
            canvas.draw(&obj.mesh, Vec2::new(arr[i].rect.x, INIT_WINDOW_SIZE.1));
            i += 1;
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) -> GameResult {
        println!("Resized screen to {width}, {height}");
        if self.window_settings.resize_projection {
            self.screen_coords = graphics::Rect::new(0.0, 0.0, width, height);
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    // TODO: (aver) add argparsing
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("algo-viz-rs", "Armin Veres")
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(INIT_WINDOW_SIZE.0, INIT_WINDOW_SIZE.1),
        )
        .window_setup(ggez::conf::WindowSetup::default().title("Sorting Algorithm Visualizer"))
        .build()?;

    let max_val = 750.;
    let no_rects = 150;
    // TODO: (aver) make sorter a parameter that initializes the state internally
    let l_sorter = BubbleSort::new(&mut ctx, max_val, no_rects);
    let state = GameState::new(l_sorter, &mut ctx)?;
    event::run(ctx, event_loop, state)
}
