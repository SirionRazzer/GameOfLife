use rand::distributions::{IndependentSample, Range};
use rand;

use rustty::{
    Size, 
    HasSize,
    CellAccessor,
    Color
};

use rustty::ui::core::{
    Widget,
    Frame,
    Painter,
    HorizontalAlign,
    VerticalAlign
};

use rustty::ui::Canvas;

// Array containing coordinates for calculating
// all adjacent neighbors nearby 
const ADJ: [(i32, i32); 8] = 
    [(-1, -1), (-1, 0), (-1, 1), 
     (0, -1), (0, 1), (1, 1), 
     (1, 0), (1, -1)];

// Each generation will decide whether a cell is
// unchanged, created, or killed
pub enum Action {
   Create,
   Kill
}

// The grid is a UI object which wraps a canvas and contains
// a vector of actions and coordinates. Areas of a canvas are
// marked with actions each iteration and actions are stored
// in the actions vector. At the end of each iteration the 
// actions vector is iterated over and all actions are applied
// to create the new generation
pub struct Grid {
    canvas: Canvas,
    actions: Vec<((usize, usize), Action)>
}

impl Grid {
    pub fn new(cols: usize, rows: usize) -> Grid {
        let mut canvas_ = Canvas::new(cols, rows);
        canvas_.draw_box();

        Grid {
            canvas: canvas_,
            actions: Vec::new()
        }
    }

    pub fn rand_color() -> Color {
        let color = Range::new(0, 6);
        let mut rng = rand::thread_rng();

        // Generate random color not including the background 
        // color of the console
        let value = color.ind_sample(&mut rng);
        Color::Byte(
            if value >= 3 {
                value + 1
            } else {
                value
            })
    }

    pub fn neighbors(&self, x: usize, y: usize) -> u8{
        let mut cnt = 0u8;
        // for every adjacent cell, check whether it's color is empty or not
        // if the color is not empty, it is alive and considered a neighbor
        for &(r, c) in &ADJ {
            let (x1, y1) = (x as i32 + r, y as i32 + c);
            if x1 > 1 && y1 > 1 {
                let (x2, y2) = (x1 as usize, y1 as usize);
                if let Some(a) = self.canvas.get(x2, y2) {
                    if a.bg() != Color::Default {
                        cnt += 1;
                    }
                }
            }
        }
        cnt
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        // create an action that will create life at this cell
        self.actions.push(
            ((x, y),
             Action::Create)
            );
    }

    pub fn set_dead(&mut self, x: usize, y: usize) {
        // create an action this will kill this cell
        self.actions.push(
            ((x, y),
             Action::Kill)
            );
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        if self.canvas.get(x, y).unwrap().bg() != Color::Default {
            return true
        }
        false
    }

    // Ensure the borders of the canvas are not used 
    pub fn playable_size(&self) -> Size {
        (self.canvas.size().0 - 1, self.canvas.size().1 - 1)
    }

    // Apply all actions
    pub fn update(&mut self) {
        while let Some(((x, y), act)) = self.actions.pop() {
            match act { 
               Action::Create   => { self.canvas.get_mut(x, y).unwrap()
                                         .set_bg(Grid::rand_color()); },
               Action::Kill     => { self.canvas.get_mut(x, y).unwrap()
                                         .set_bg(Color::Default); }
            }
        }
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }
}

impl Widget for Grid {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.canvas.draw(parent);
    }

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
            margin: (usize, usize)) {
        self.canvas.pack(parent, halign, valign, margin);
    }

    fn resize(&mut self, new_size: Size) {
        self.canvas.resize(new_size);
    }

    fn draw_box(&mut self) {
        self.canvas.draw_box();
    }

    fn frame(&self) -> &Frame {
        self.canvas.frame()
    }

    fn frame_mut(&mut self) -> &mut Frame {
        self.canvas.frame_mut()
    }
}
