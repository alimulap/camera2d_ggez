use ggez::{
    event::{EventHandler, MouseButton},
    graphics::{Canvas, Color, DrawMode, DrawParam, FillOptions, Mesh, Rect},
    winit::event::VirtualKeyCode,
    Context, GameResult,
};
use camera2d_ggez::Camera;

mod app;
use app::example;

fn main() -> GameResult {
    example::run("Mesh", |ctx| MeshExample::new(ctx))
}

struct MeshExample {
    camera: Camera,
    text: example::TextBox,
    mesh1: example::DrawableWrapper<Mesh>,
    mesh2: example::DrawableWrapper<Mesh>,
    mesh3: Mesh,
    mesh4: Mesh,
}

impl MeshExample {
    fn new(ctx: &Context) -> Self {
        let mesh1 = example::DrawableWrapper {
            content: Mesh::new_circle(
                ctx,
                DrawMode::Fill(FillOptions::default()),
                [100., 100.],
                100.,
                0.5,
                Color::CYAN,
            )
            .unwrap(),
            tf: camera2d_ggez::Transform {
                dest: [100., 100.].into(),
                ..Default::default()
            },
        };

        let mesh2 = example::DrawableWrapper {
            content: Mesh::new_rectangle(
                ctx,
                DrawMode::Fill(FillOptions::default()),
                Rect::new(0., 0., 200., 200.),
                Color::MAGENTA,
            )
            .unwrap(),
            tf: camera2d_ggez::Transform {
                dest: [350., 50.].into(),
                ..Default::default()
            },
        };

        let mesh3 = Mesh::new_polygon(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            &[[15., 60.], [35., 60.], [15., 80.]],
            Color::WHITE,
        )
        .unwrap();

        let screen_dim = ctx.gfx.drawable_size();

        let mesh4 = Mesh::new_polygon(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            &[
                [screen_dim.0 - 15., screen_dim.1 - 10.],
                [screen_dim.0 - 35., screen_dim.1 - 10.],
                [screen_dim.0 - 15., screen_dim.1 - 30.],
            ],
            Color::WHITE,
        )
        .unwrap();

        let text = example::TextBox::new(
            ctx,
            "Hold and drag / hold space to move camera\nScrool to zoom",
            Rect::new(10., 10., 380., 40.),
        );

        MeshExample {
            camera: Camera::default(),
            text,
            mesh1,
            mesh2,
            mesh3,
            mesh4,
        }
    }
}

impl EventHandler for MeshExample {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 30));
        let params = DrawParam::default();

        self.mesh1
            .draw(&mut canvas, params, &self.camera.to_matrix());

        let camera_matrix = self.camera.to_matrix();
        canvas.draw(
            &self.mesh2.content,
            params.transform(self.mesh2.tf.apply_matrix(&camera_matrix)),
        );

        canvas.draw(&self.mesh3, self.camera);
        canvas.draw(&self.mesh4, params);

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
