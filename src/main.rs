use clap::{Parser, ValueEnum};
use ggez::{
    event,
    glam::Vec2,
    graphics::{self, Rect},
    Context, GameResult,
};
use sorting::{BubbleSort, InsertionSort, SelectionSort, Sorter, INIT_WINDOW_SIZE};
use std::{sync::mpsc, thread, time};

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
    desired_fps: u32,
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
            desired_fps,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.sorter.do_check() && self.sorter.is_sorted() {
            self.sorter.check_step();
        } else if !self.sorter.do_check() && !self.sorter.is_sorted() {
            // only update in given steps per second
            while ctx.time.check_update_time(self.desired_fps) {
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
        canvas.set_screen_coordinates(self.screen_coords); // set custom canvas for resizing

        let arr = self.sorter.get_arr();
        // draw each mesh with the coordinates being the adjusted x coordinates and the width of
        // the default resolution, which gets scaled, when resizing
        for obj in &mut *arr {
            canvas.draw(&obj.mesh, Vec2::new(obj.rect.x, INIT_WINDOW_SIZE.1));
        }

        // Add name of sorting algorithm in top left corner
        let text = graphics::Text::new(self.sorter.get_name());
        canvas.draw(&text, Vec2::new(0., 0.));

        canvas.finish(ctx)?;

        // Update all underlying meshes once again after drawing, since some bars stay red, even
        // after going through them in the swap function
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
use rodio::{source::SineWave, OutputStream, Sink, Source};

/// Range in between which the audio beep is played
const AUDIO_RANGE_HZ: (f32, f32) = (100., 1000.);

pub fn main() -> GameResult {
    let args = CLIArgs::parse();
    // add channels for audio communcations
    let (tx, rx): (mpsc::Sender<f32>, mpsc::Receiver<f32>) = mpsc::channel();
    // Setup rodio audio parts
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let _audio_thread = thread::spawn(move || {
        let slope = (AUDIO_RANGE_HZ.1 - AUDIO_RANGE_HZ.0) / (args.max_val);
        loop {
            // Receive data from the channel
            // match rx.recv_timeout(time::Duration::from_millis(50)) {
            match rx.recv() {
                //Use the received data to generate the audio
                Ok(received_data) => {
                    // Normalize range to 100-1000 Hz
                    let frequency = AUDIO_RANGE_HZ.0 + (slope * (received_data));
                    let duration = time::Duration::from_millis(50); // Set the duration of the tone
                    let source = SineWave::new(frequency).take_duration(duration);

                    sink.append(source);
                    sink.sleep_until_end();
                }
                Err(_) => {
                    //Handle timeout or errors
                }
            }
        }
    });

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

    event::run(ctx, event_loop, state);
    _audio_thread.join().unwrap()
}
