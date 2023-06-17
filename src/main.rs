extern crate rand;
extern crate rgb;

use rand::Rng;
use rgb::RGB8;
use std::f64::consts::PI;
use std::thread;
use std::time::Duration;

struct Main {
    window_width: u32,
    window_height: u32,
    line_length: f64,
    root: iced::window::Window,
    canvas: iced::canvas::Canvas<()>,
}

impl Main {
    fn new() -> Main {
        // initalise the main structure with default values
        let window_width = 1920;
        let window_height = 1080;
        let line_length = 0.0;
        let root = iced::window::Window::new();
        let canvas = iced::canvas::Canvas::new().width(window_width).height(window_height);

        Main {
            window_width,
            window_height,
            line_length,
            root,
            canvas,
        }
    }

    fn update(&mut self) {
        // increment the line length by 2
        self.line_length += 2.0;
        // clear the canvas
        self.canvas.clear();
        // fill the canvas with black color
        self.canvas.draw(|frame, _| {
            let bounds = frame.bounds();
            frame.fill_rectangle((0.0, 0.0), bounds, iced::Color::BLACK);
        });
    
        // set the line length, angle, x and y coordinates, length step and angle step
        let line_length = self.line_length;
        let line_angle = 0.0;
        let x = 960.0;
        let y = 840.0;
        let length_step = 10.0;
        let line_angle_step = 30.0;

        self.draw_stick(line_length, line_angle, x, y, length_step, line_angle_step);
        // repaint the root window
        self.root.repaint();
        // wait for 13 milliseconds
        thread::sleep(Duration::from_millis(13));
        self.update();
    }

    fn draw_stick(&mut self, line_length: f64, line_angle: f64, x: f64, y: f64, length_step: f64, line_angle_step: f64) {
        // calculate the size of the line in x and y dimensions
        let x_size = line_length * (line_angle - 90.0).to_radians().cos();
        let y_size = line_length * (line_angle - 90.0).to_radians().sin();
        
        // generate random color
        let color = RGB8::new(
            rand::thread_rng().gen_range(0..255),
            rand::thread_rng().gen_range(0..255),
            rand::thread_rng().gen_range(0..255),
        );
        
        // draw the line on the empty canvas
        self.canvas.draw(|frame, _| {
            frame.stroke(
                &iced::widget::line::Segment::new((x, y), (x + x_size, y + y_size)),
                iced::widget::line::Options::default().with_stroke_width(1.0).with_color(color),
            );
        });

        // recursive function to draw branches of the tree
        if line_length >= 1.0 {
            self.draw_stick(
                line_length - length_step,
                line_angle - line_angle_step,
                x + x_size,
                y + y_size,
                length_step,
                line_angle_step,
            );
            self.draw_stick(
                line_length - length_step,
                line_angle + line_angle_step,
                x + x_size,
                y + y_size,
                length_step,
                line_angle_step,
            );
        }
    }
}

fn main() {
    iced::run::<Main>(iced::Settings::default())
        .expect("Failed to run the application");
}