# camera_ggez
Camera implementation in https://github.com/ggez/ggez/

## What is this?

It is not camera like some people probably thought of 
but it is just a transformation matrix that can be applied to drawable object 
so (i think) any drawable in [ggez](https://github.com/ggez/ggez/) object can be affected by this camera matrix.

## example

```rust
struct MainState {
    camera: Camera,
    mesh: Mesh,
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 30));

        canvas.draw(&self.mesh, self.camera);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        self.camera.zoom_at_screen_coords(ctx.mouse.position(), [1. + 0.1 * y, 1. + 0.1 * y]);
        Ok(())
    }
}
```
