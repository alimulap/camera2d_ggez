#![allow(unused)]

pub mod example {
    use std::{
        env, 
        path::PathBuf
    };

    use ggez::{
        event, 
        Context,
        GameResult, 
        glam::Mat4, 
        mint::Vector2, 
        ContextBuilder,
        conf::{WindowMode, WindowSetup}, 
        graphics::{self, Mesh, TextFragment, Rect, DrawMode, FillOptions, Color, Canvas, DrawParam, Drawable}, 
    };

    pub fn run<F, E>(name: &str, init: F) -> GameResult
    where F: FnOnce(&mut Context) -> E,
          E: event::EventHandler + 'static
    {
        env::set_var("RUST_BACKTRACE", "full");
        let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = PathBuf::from(manifest_dir);
            path.push("resources");
            path
        } else {
            PathBuf::from("./resources")
        };

        let cb = ContextBuilder::new("Camera2D", "alimulap")
            .window_mode(WindowMode {
                width: 720.,
                height: 480.,
                resizable: true,
                ..Default::default() })
            .window_setup(WindowSetup { 
                title: name.to_owned(), 
                ..Default::default() })
            .add_resource_path(resource_dir);
        let (mut ctx, event_loop) = cb.build()?;
        let state = init(&mut ctx);
        event::run(ctx, event_loop, state)
    }

    pub struct DrawableWrapper<D: Drawable> {
        pub content: D,
        pub tf: camera2d_ggez::Transform,
    }

    impl<D: Drawable> DrawableWrapper<D> {
        pub fn draw(&mut self, canvas: &mut Canvas, params: DrawParam, parent_matrix: &Mat4) {
            canvas.draw(
                &self.content, 
                params.transform(self.tf.apply_matrix(parent_matrix)));
        }
    }

    pub struct TextBox {
        content: graphics::Text,
        text_pos: [f32; 2],
        background: Mesh,
    }

    impl TextBox {
        pub fn new(ctx: &Context, text: impl Into<TextFragment>, rect: Rect) -> Self {
            let background = Mesh::new_rectangle(
                ctx, DrawMode::Fill(FillOptions::default()), 
                rect, Color::from_rgba(40, 40, 40, 200)).unwrap();

            let mut content = graphics::Text::new(text);
            //content.set_wrap(false);
            //content.set_bounds(Vector2::from_slice(&[rect.w * 0.8, rect.h * 0.8]));

            let pad = if rect.w < rect.h { rect.w * 0.1 } else { rect.h * 0.1 };
            let text_pos = [rect.x + pad, rect.y + pad];

            TextBox { 
                content, 
                text_pos,
                background, 
            }

        }

        pub fn draw(&mut self, ctx: &Context, canvas: &mut Canvas) {
            let params = DrawParam::default();
            canvas.draw(&self.background, params);
            canvas.draw(&self.content, params.dest(self.text_pos));
        }
    }
}

fn main() { }
