use ggez::{
    Context, 
    ContextBuilder,
    GameResult,
    conf::WindowMode,
    graphics::{
        Mesh, DrawMode, Color, FillOptions, Canvas, DrawParam, Transform}, 
    event::{self, EventHandler, MouseButton}, winit::event::VirtualKeyCode, 
    input::keyboard::KeyInput, mint::Vector2,
};
mod camera;
use camera::Camera;

pub struct Game {
    camera: Camera,
    circle1: Mesh,
    circle2: Mesh,
    tf1: Transform,
    tf2: Transform,
    should_move: bool,
}

impl Game {
    pub fn new(ctx: &Context) -> GameResult<Self> {
        let circle1  = Mesh::new_circle(
            ctx, DrawMode::Fill(FillOptions::default()), 
            [0., 0.], 100., 1., Color::CYAN).unwrap();
        let circle2  = Mesh::new_circle(
            ctx, DrawMode::Fill(FillOptions::default()), 
            [0., 0.], 100., 1., Color::MAGENTA).unwrap();
        let tf1 = Transform::Values { dest: [100., 100.].into(), rotation: 0., scale: [1., 1.].into(), offset: [0., 0.].into() };
        let tf2 = Transform::Values { dest: [450., 100.].into(), rotation: 0., scale: [1., 1.].into(), offset: [0., 0.].into() };

        Ok(Game{
            camera: Camera::default(),
            circle1,
            circle2,
            tf1,
            tf2,
            should_move: false,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(10, 10, 10));

        canvas.draw(&self.circle1, DrawParam::default().transform(self.camera.apply_with(&self.tf1)));
        canvas.draw(&self.circle2, DrawParam::default().transform(self.camera.apply_with(&self.tf2)));

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        match input.keycode {
            Some(VirtualKeyCode::Escape) => ctx.request_quit(),
            Some(VirtualKeyCode::W) => {
                if let Transform::Values { scale, .. } = &mut self.tf1 {
                    scale.x += scale.x * 0.1;
                    scale.y += scale.y * 0.1;
                }
            },
            Some(VirtualKeyCode::S) => {
                if let Transform::Values { scale, .. } = &mut self.tf1 {
                    scale.x -= scale.x * 0.1;
                    scale.y -= scale.y * 0.1;
                }
            },
            Some(VirtualKeyCode::Up) => {
                if let Transform::Values { scale, .. } = &mut self.tf2 {
                    scale.x += scale.x * 0.1;
                    scale.y += scale.y * 0.1;
                }
            },
            Some(VirtualKeyCode::Down) => {
                if let Transform::Values { scale, .. } = &mut self.tf2 {
                    scale.x -= scale.x * 0.1;
                    scale.y -= scale.y * 0.1;
                }
            },
            _ => (),
        }
        Ok(())
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        let zoom_factor = Vector2 {
            x: 1. - y * 0.1f32,
            y: 1. - y * 0.1f32,
        };
        self.camera.zoom_at_screen_coords(ctx.mouse.position(), zoom_factor);
        // self.camera.zoom_center(ctx, [1. + y * 0.1, 1. + y * 0.1]);
        // self.camera.zoom([1. + y * 0.1, 1. + y * 0.1]);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> GameResult {
        if button == MouseButton::Left {
            self.should_move = true;
        }
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> GameResult {
        if button == MouseButton::Left {
            self.should_move = false;
        }
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, dx: f32, dy: f32) -> GameResult {
        if self.should_move { 
            // move the camera
            self.camera.move_by_screen_coords([dx, dy]); 
            // self.camera.move_by_world_coords([dx, dy]); 

            println!("In screen coords:");
            if let Transform::Values { dest, .. } = &mut self.tf1 {
                // move in world
                // dest.x += dx;
                // dest.y += dy;

                let pos = self.camera.world_to_screen_coords(*dest);
                println!("Circle 1:");
                println!("Screen coords: x:{} y:{}", pos.x, pos.y);
                println!("World coords: x:{} y:{}", dest.x, dest.y);
            }
            if let Transform::Values { dest, .. } = &mut self.tf2 {
                // move in world
                // dest.x += dx;
                // dest.y += dy;

                let pos = self.camera.world_to_screen_coords(*dest);
                println!("Circle 2:");
                println!("Screen coords: x:{} y:{}", pos.x, pos.y);
                println!("World coords: x:{} y:{}", dest.x, dest.y);
            }
            println!("");
        } 
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("Camera", "alimulap")
        .window_mode(WindowMode::default().dimensions(720., 480.));
    let (mut ctx, event_loop) = cb.build()?;
    let state = Game::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
