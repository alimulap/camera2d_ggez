use ggez::{
    event::{EventHandler, MouseButton},
    glam::Vec2,
    graphics::{Canvas, Color, DrawParam, Image, Rect},
    mint::Point2,
    winit::event::VirtualKeyCode,
    Context, GameResult,
};
use camera2d_ggez::Camera;

mod app;
use app::example;

fn main() -> GameResult {
    example::run("Image", |ctx| ImageExample::new(ctx))
}

struct ImageExample {
    camera: Camera,
    text: example::TextBox,
    image: example::DrawableWrapper<Image>,
}

impl ImageExample {
    fn new(ctx: &Context) -> Self {
        let mut image = example::DrawableWrapper {
            content: Image::from_path(ctx, "/subaru-duck.png").unwrap(),
            tf: camera2d_ggez::Transform::default(),
        };

        image.tf = camera2d_ggez::Transform {
            dest: (Vec2::from(ctx.gfx.drawable_size()) / 2.).into(),
            offset: Point2 {
                x: image.content.width() as f32 / 2.,
                y: image.content.height() as f32 / 2.,
            },
            ..Default::default()
        };

        let text = example::TextBox::new(
            ctx,
            "Hold and drag / hold space to move camera\nScrool to zoom",
            Rect::new(10., 10., 380., 40.),
        );

        ImageExample {
            camera: Camera::default(),
            text,
            image,
        }
    }
}

impl EventHandler for ImageExample {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 30));

        let params = DrawParam::default();

        self.image
            .draw(&mut canvas, params, &self.camera.to_matrix());

        self.text.draw(ctx, &mut canvas);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        _x: f32,
        _y: f32,
        dx: f32,
        dy: f32,
    ) -> GameResult {
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::Space)
            || ctx.mouse.button_pressed(MouseButton::Left)
        {
            self.camera.move_by_screen_coords([dx, dy]);
        }
        Ok(())
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        self.camera
            .zoom_at_screen_coords(ctx.mouse.position(), [1. + 0.1 * y, 1. + 0.1 * y]);
        Ok(())
    }
}
