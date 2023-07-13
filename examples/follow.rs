use ggez::{
    Context, 
    GameResult,
    event::EventHandler, 
    winit::event::VirtualKeyCode, 
    glam::Vec2, mint::Point2, 
    graphics::{ Color, Canvas, DrawParam, Image, Mesh, DrawMode, FillOptions, Rect, Sampler }
};
use camera2d_ggez::Camera;

mod app;
use app::example;

fn main() -> GameResult {
    example::run("Follow", |ctx|{ FollowExample::new(ctx) })
}

struct FollowExample {
    camera: Camera,
    player: example::DrawableWrapper<Mesh>,
    map: example::DrawableWrapper<Image>,
    text: example::TextBox,
}

impl FollowExample {
    fn new(ctx: &Context) -> Self {
        let map = example::DrawableWrapper {
            content: Image::from_path(ctx, "/earth.motfe.net.png").unwrap(),
            tf: camera2d_ggez::Transform {
                scale: [3., 3.].into(),
                ..Default::default() }};

        let player = example::DrawableWrapper {
            content: Self::create_triangle(ctx, 15.),
            tf: camera2d_ggez::Transform {
                dest: Point2 {
                    x: map.content.width() as f32 / 2. * map.tf.scale.x,
                    y: map.content.height() as f32 / 2. * map.tf.scale.y
                },
                ..Default::default() }};

        let mut camera = Camera::default();
        camera.set_offset(Vec2::from(ctx.gfx.drawable_size()) / 2.);
        camera.set_position(player.tf.dest);

        let text = example::TextBox::new(
            ctx, 
            "Press Left or Right to rotate player\nPress space to move player\nScrool to zoom\nPress A or D to rotate camera", 
            Rect::new(20., 20., 350., 90.));

        FollowExample { 
            camera, 
            player,
            map,
            text,
        }
    }

    fn create_triangle(ctx: &Context, roc: f32) -> Mesh {
        Mesh::new_polygon(ctx, DrawMode::Fill(FillOptions::default()), 
            &[
                Point2 {
                    x: 0f32.to_radians().cos() * (roc * 2.),
                    y: 0f32.to_radians().sin() * (roc * 2.),
                },
                Point2 {
                    x: 120f32.to_radians().cos() * roc,
                    y: 120f32.to_radians().sin() * roc,
                },
                Point2 {
                    x: 240f32.to_radians().cos() * roc,
                    y: 240f32.to_radians().sin() * roc,
                },
            ], 
            Color::RED)
        .unwrap()
    }
}

impl EventHandler for FollowExample {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        use VirtualKeyCode as Key;
        if ctx.keyboard.is_key_pressed(Key::Right) { self.player.tf.rotation += 0.1; }
        if ctx.keyboard.is_key_pressed(Key::Left) { self.player.tf.rotation -= 0.1; }
        if ctx.keyboard.is_key_pressed(Key::Space) {
            let delta = Point2 {
                x: self.player.tf.rotation.cos() * 10.,
                y: self.player.tf.rotation.sin() * 10.,
            };

            self.player.tf.dest = Point2 {
                x: self.player.tf.dest.x + delta.x,
                y: self.player.tf.dest.y + delta.y,
            };
            self.camera.set_position(self.player.tf.dest);
            //self.camera.move_by_world_coords(delta);
        }

        if ctx.keyboard.is_key_pressed(Key::A) { self.camera.rotate(0.01); }
        if ctx.keyboard.is_key_pressed(Key::D) { self.camera.rotate(-0.01); }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 30));
        if self.camera.get_zoom().gt(&[0.6, 0.6].into()) {
            canvas.set_sampler(Sampler::nearest_clamp());
        }

        let params = DrawParam::default();
        let camera_matrix = self.camera.to_matrix();

        self.map.draw(&mut canvas, params, &camera_matrix);
        self.player.draw(&mut canvas, params, &camera_matrix);
        self.text.draw(ctx, &mut canvas);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        self.camera.zoom_center(ctx, [1. + 0.1 * y, 1. + 0.1 * y]);
        Ok(())
    }
}

