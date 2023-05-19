use clap::{Parser, ValueEnum};
use ggez::{
    event,
    glam::Vec2,
    graphics::{self, Rect},
    Context, GameResult,
};
use sorting::{
    start_audio_thread, BubbleSort, InsertionSort, SelectionSort, Sorter, INIT_WINDOW_SIZE,
};
use std::sync::mpsc;

#[derive(Clone, ValueEnum)]
enum SortingAlgorithms {
    Bubblesort,
    Insertionsort,
    Selectionsort,
}

#[derive(Parser)]
struct CLIArgs {
    #[arg(short, long, default_value_t = 1000.)]
    max_val: f32,
    #[arg(short, long, default_value_t = 150)]
    no_rects: u32,
    #[arg(short, long, default_value_t = 10)]
    steps_per_second: u32,
    #[arg(value_enum, default_value_t = SortingAlgorithms::Bubblesort)]
    sorting_algo: SortingAlgorithms,
}

struct WindowSettings {
    resize_projection: bool,
}

struct GameState {
    frames: usize,
    window_settings: WindowSettings,
    screen_coords: Rect,
    sorter: Box<dyn Sorter>,
    /// Alias to steps per second
    target_fps: u32,
}

impl GameState {
    fn new(
        given_sorter: Box<dyn Sorter>,
        ctx: &mut Context,
        desired_fps: u32,
    ) -> GameResult<GameState> {
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
            target_fps: desired_fps,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Only draw after timeout for fps
        while ctx.time.check_update_time(self.target_fps) {
            // check if we want to step in the algorithm or the final test/check
            if self.sorter.do_check() && self.sorter.is_sorted() {
                self.sorter.check_step();
            } else if !self.sorter.do_check() && !self.sorter.is_sorted() {
                // only update in given steps per second
                self.sorter.step();
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

        // set custom canvas for resizing
        canvas.set_screen_coordinates(self.screen_coords);

        let arr = self.sorter.get_arr();

        // draw each mesh with the coordinates being the adjusted x coordinates and the width of
        // the default resolution, which gets scaled, when resizing
        for obj in &mut *arr {
            canvas.draw(&obj.mesh, Vec2::new(obj.rect.x, INIT_WINDOW_SIZE.1));
        }

        // Add name of sorting algorithm and delay in ms to top left corner
        let delay = 1000. / self.target_fps as f32;
        let text = graphics::Text::new(format!("{}, delay: {} ms", self.sorter.get_name(), delay));
        canvas.draw(&text, Vec2::new(0., 0.));

        canvas.finish(ctx)?;

        // Update all underlying meshes only after drawing
        for elem in self.sorter.get_arr().iter_mut() {
            let old_rect = elem.rect;
            (*elem).mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, old_rect.w, old_rect.h),
                elem.sort_state.get_color(),
            )
            .unwrap();
        }

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
    let args = CLIArgs::parse();
    // add channels for audio communcations
    let (tx, rx): (mpsc::Sender<f32>, mpsc::Receiver<f32>) = mpsc::channel();

    start_audio_thread(rx, args.max_val, args.steps_per_second);

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("algo-viz-rs", "Armin Veres")
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(INIT_WINDOW_SIZE.0, INIT_WINDOW_SIZE.1),
        )
        .window_setup(ggez::conf::WindowSetup::default().title("Sorting Algorithm Visualizer"))
        .build()?;

    let sorter: Box<dyn Sorter> = match args.sorting_algo {
        SortingAlgorithms::Bubblesort => {
            Box::new(BubbleSort::new(&mut ctx, args.max_val, args.no_rects, tx))
        }
        SortingAlgorithms::Insertionsort => Box::new(InsertionSort::new(
            &mut ctx,
            args.max_val,
            args.no_rects,
            tx,
        )),
        SortingAlgorithms::Selectionsort => Box::new(SelectionSort::new(
            &mut ctx,
            args.max_val,
            args.no_rects,
            tx,
        )),
    };
    let state = GameState::new(sorter, &mut ctx, args.steps_per_second)?;
    event::run(ctx, event_loop, state)
    // _audio_thread.join().unwrap() // unreachable since even::run handles game
}
