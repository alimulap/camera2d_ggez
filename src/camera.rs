use ggez::{
    glam::{Mat4, Vec3},
    graphics::DrawParam,
    mint::{Point2, Vector2},
    Context,
};

use super::transform::Transform;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub offset: Point2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
    pub position: Point2<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            offset: Point2 { x: 0., y: 0. },
            rotation: 0.,
            scale: Vector2 { x: 1., y: 1. },
            position: Point2 { x: 0., y: 0. },
        }
    }
}

impl Camera {
    pub fn new<P, V>(offset: P, rotation: f32, scale: V, position: P) -> Self
    where
        P: Into<Point2<f32>> + Copy,
        V: Into<Vector2<f32>>,
    {
        Camera {
            offset: offset.into(),
            rotation,
            scale: scale.into(),
            position: position.into(),
        }
    }
    pub fn to_matrix(&self) -> Mat4 {
        let (sinr, cosr) = self.rotation.sin_cos();
        let m00 = cosr * self.scale.x;
        let m01 = -sinr * self.scale.y;
        let m10 = sinr * self.scale.x;
        let m11 = cosr * self.scale.y;
        let m03 = self.position.x * (-m00) - self.position.y * m01 + self.offset.x;
        let m13 = self.position.y * (-m11) - self.position.x * m10 + self.offset.y;

        Mat4::from_cols_array(&[
            m00, m01, 0.0, m03, //
            m10, m11, 0.0, m13, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,//
        ])
        .transpose()
    }

    pub fn apply_matrix<T>(&self, object: T) -> Mat4
    where
        T: Into<Transform>,
    {
        let object: Transform = object.into();

        self.to_matrix().mul_mat4(&object.to_matrix())
    }

    pub fn world_to_screen_coords<P>(&self, point: P) -> Point2<f32>
    where
        P: Into<Point2<f32>>,
    {
        let point: Point2<f32> = point.into();
        let point = Vec3::new(point.x, point.y, 0.);
        let screen_point = self.to_matrix().transform_point3(point);
        Point2 {
            x: screen_point.x,
            y: screen_point.y,
        }
    }

    pub fn screen_to_world_coords<P>(&self, point: P) -> Point2<f32>
    where
        P: Into<Point2<f32>>,
    {
        let inverse_matrix = self.to_matrix().inverse();
        let point: Point2<f32> = point.into();
        let point = Vec3::new(point.x, point.y, 0.);
        let world_point = inverse_matrix.transform_point3(point);
        Point2 {
            x: world_point.x,
            y: world_point.y,
        }
    }

    pub fn set_position<P>(&mut self, point: P)
    where
        P: Into<Point2<f32>>,
    {
        let point: Point2<f32> = point.into();
        self.position.x = point.x;
        self.position.y = point.y;
    }

    pub fn set_offset<P>(&mut self, point: P)
    where
        P: Into<Point2<f32>>,
    {
        let point: Point2<f32> = point.into();
        self.offset.x = point.x * self.scale.x;
        self.offset.y = point.y * self.scale.y;
    }

    pub fn move_by_world_coords<P>(&mut self, delta: P)
    where
        P: Into<Point2<f32>>,
    {
        let delta: Point2<f32> = delta.into();
        self.position.x -= delta.x;
        self.position.y -= delta.y;
    }

    pub fn move_by_screen_coords<P>(&mut self, delta: P)
    where
        P: Into<Point2<f32>>,
    {
        let delta: Point2<f32> = delta.into();
        self.position.x -= delta.x / self.scale.x;
        self.position.y -= delta.y / self.scale.y;
    }

    pub fn get_zoom(&self) -> Vector2<f32> {
        self.scale
    }

    pub fn set_zoom<V>(&mut self, scale: V)
    where
        V: Into<Vector2<f32>>,
    {
        self.scale = scale.into();
    }

    pub fn zoom<V>(&mut self, factor: V)
    where
        V: Into<Vector2<f32>>,
    {
        let factor: Vector2<f32> = factor.into();
        self.scale.x *= factor.x;
        self.scale.y *= factor.y;
    }

    pub fn zoom_center<V>(&mut self, ctx: &Context, factor: V)
    where
        V: Into<Vector2<f32>>,
    {
        let factor: Vector2<f32> = factor.into();
        let screen_rect = ctx.gfx.drawable_size();
        let screen_center = Point2 {
            x: screen_rect.0 / 2.0,
            y: screen_rect.1 / 2.0,
        };
        let world_center = self.screen_to_world_coords(screen_center);
        self.position.x = world_center.x - (world_center.x - self.position.x) / factor.x;
        self.position.y = world_center.y - (world_center.y - self.position.y) / factor.y;
        self.scale.x *= factor.x;
        self.scale.y *= factor.y;
    }

    pub fn zoom_at_screen_coords<P, V>(&mut self, point: P, factor: V)
    where
        P: Into<Point2<f32>>,
        V: Into<Vector2<f32>>,
    {
        let point: Point2<f32> = point.into();
        let factor: Vector2<f32> = factor.into();
        let world_center = self.screen_to_world_coords(point);
        self.position.x = world_center.x - (world_center.x - self.position.x) / factor.x;
        self.position.y = world_center.y - (world_center.y - self.position.y) / factor.y;
        self.scale.x *= factor.x;
        self.scale.y *= factor.y;
    }

    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }

    pub fn set_rotation(&mut self, angle: f32) {
        self.rotation = angle;
    }
}

impl From<Camera> for DrawParam {
    fn from(value: Camera) -> Self {
        DrawParam::default().transform(value.to_matrix())
    }
}
