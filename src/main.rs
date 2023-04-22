mod sorting;

use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

use rand::{self, Rng};

// TODO: (aver) implement dynamic screen scaling, as in the snake example
const RESOLUTION: (u32, u32) = (2250, 900);
const NO_RECTS: usize = 150;
const DESIRED_FPS: u32 = 10;
const MAX_VALUE: f32 = 750.0;

pub trait Sorter {
    /// Common step interface for sorting algorithms
    fn step(&mut self, ctx: &Context);
    fn get_arr(&self) -> &Vec<SortElement>;
    fn is_sorted(&self) -> bool;
}

struct BubbleSort {
    arr: Vec<SortElement>,
    sorted: bool,
    outer_index: usize,
}

impl BubbleSort {
    fn new(ctx: &mut Context) -> Self {
        let mut sort_elems = vec![];
        for i in 0..NO_RECTS {
            sort_elems.push(SortElement::new(ctx, i).unwrap());
        }
        Self {
            sorted: false,
            arr: sort_elems,
            outer_index: NO_RECTS,
        }
    }
}

impl Sorter for BubbleSort {
    fn step(&mut self, ctx: &Context) {
        // currently if there is not step left between inner and outer index, the step will be
        // empty
        self.sorted = true;
        for i in 0..(self.outer_index - 1) {
            if self.arr[i].get_height() > self.arr[i + 1].get_height() {
                self.sorted = false; // if we meet another value, we obviously are unsorted
                let one = self.arr[i].rect;
                let two = self.arr[i + 1].rect;
                self.arr[i].rect.h = two.h;
                self.arr[i + 1].rect.h = one.h;

                self.arr[i].mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(one.x, one.y, one.w, two.h),
                    // self.arr[i + 1].state.get_color(),
                    Color::GREEN,
                )
                .unwrap();

                self.arr[i + 1].mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(two.x, two.y, two.w, one.h),
                    self.arr[i].state.get_color(),
                )
                .unwrap();
            }
        }
        self.outer_index -= 1;
        return;
    }

    fn get_arr(&self) -> &Vec<SortElement> {
        &self.arr
    }

    fn is_sorted(&self) -> bool {
        self.sorted
    }
}

#[derive(Default)]
enum SortState {
    #[default]
    UNSORTED,
    SORTED,
    SELECTED,
}

impl SortState {
    fn get_color(&self) -> Color {
        match self {
            Self::UNSORTED => Color::WHITE,
            Self::SORTED => Color::GREEN,
            Self::SELECTED => Color::RED,
        }
    }
}

// TODO: implement color change, currently it looks like we would have to replace the mesh with a
// new one, that has the updated color.
pub struct SortElement {
    /// Mesh that gets drawn
    mesh: graphics::Mesh,
    /// Part that will get sorted
    rect: graphics::Rect,
    /// Holds sorting state
    state: SortState,
}

impl SortElement {
    fn new(ctx: &mut Context, i: usize) -> GameResult<Self> {
        const SCALE: f32 = 0.75;
        const X_OFFSET: f32 = 10.;
        let mut rng = rand::thread_rng();

        let l_state = SortState::default();

        let mut l_rectangle = graphics::Rect::new(0., 100., 10., 0.);

        // we need to reverse the height, since origin is at top left
        l_rectangle.h = -rng.gen_range(0.0..MAX_VALUE);
        l_rectangle.x += X_OFFSET + i as f32 * l_rectangle.w * SCALE;

        let l_object = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            l_rectangle,
            l_state.get_color(),
        )?;

        Ok(Self {
            mesh: l_object,
            rect: l_rectangle,
            state: l_state,
        })
    }

    fn get_height(&self) -> f32 {
        -self.rect.h
    }

    // fn swap_with(&mut self, other: &mut SortElement, ctx: &Context) {
    //     let old_mesh = other.mesh.clone();
    //     other.mesh = graphics::Mesh::new_rectangle(
    //         ctx,
    //         graphics::DrawMode::fill(),
    //         self.rect,
    //         self.state.get_color(),
    //     )
    //     .unwrap();
    //     self.mesh = old_mesh;
    // }
}

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
