use ggez::{
    mint::{Point2, Vector2, ColumnMatrix4}, 
    glam::Mat4, 
    graphics::Transform, 
    Context
};
use nalgebra;

pub struct Camera {
    dest: Point2<f32>,
    rotation: f32,
    scale: Vector2<f32>,
    offset: Point2<f32>
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            dest: Point2 { x: 0., y: 0. },
            rotation: 0.,
            scale: Vector2 { x: 1., y: 1. },
            offset: Point2 { x: 0., y: 0. },
        }
    }
}

impl Camera {
    pub fn to_matrix(&self) -> ColumnMatrix4<f32> {
        let (sinr, cosr) = self.rotation.sin_cos();
        let m00 = cosr * self.scale.x;
        let m01 = -sinr * self.scale.y;
        let m10 = sinr * self.scale.x;
        let m11 = cosr * self.scale.y;
        let m03 = self.offset.x * (-m00) - self.offset.y * m01 + self.dest.x;
        let m13 = self.offset.y * (-m11) - self.offset.x * m10 + self.dest.y;

        Mat4::from_cols_array(&[
            m00, m01, 0.0, m03,
            m10, m11, 0.0, m13,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]).transpose().into()
    }

    pub fn apply_with(&self, object: &Transform) -> ColumnMatrix4<f32>{
        return (
            Mat4::from(self.to_matrix())
            .mul_mat4(
            &Mat4::from(object.to_bare_matrix()))
        ).into();
    }

    pub fn world_to_screen_coords<P>(&self, point: P) -> Point2<f32> 
        where P: Into<Point2<f32>>
    {
        let matrix = nalgebra::Matrix4::from(self.to_matrix());
        let point: Point2<f32> = point.into();
        let point = nalgebra::Point3::new(point.x, point.y, 0.0);
        let screen_point = matrix.transform_point(&point);
        Point2 {
            x: screen_point.x,
            y: screen_point.y,
        }
    }

    pub fn screen_to_world_coords<P>(&self, point: P) -> Point2<f32> 
        where P: Into<Point2<f32>>
    {
        let matrix = nalgebra::Matrix4::from(self.to_matrix());
        let inverse_matrix = matrix.try_inverse().unwrap();
        let point: Point2<f32> = point.into();
        let point = nalgebra::Point3::new(point.x, point.y, 0.0);
        let world_point = inverse_matrix.transform_point(&point);
        Point2 {
            x: world_point.x,
            y: world_point.y,
        }
    }

    pub fn move_by_world_coords<P>(&mut self, delta: P) 
        where P: Into<Point2<f32>>,
    {
        let delta: Point2<f32> = delta.into();
        self.offset.x -= delta.x;
        self.offset.y -= delta.y;
    }

    pub fn move_by_screen_coords<P>(&mut self, delta: P) 
        where P: Into<Point2<f32>>,
    {
        let delta: Point2<f32> = delta.into();
        self.offset.x -= self.scale.x * delta.x;
        self.offset.y -= self.scale.y * delta.y;
    }

    pub fn zoom<V>(&mut self, factor: V) 
        where V: Into<Vector2<f32>>
    {
        let factor: Vector2<f32> = factor.into();
        self.scale.x *= factor.x;
        self.scale.y *= factor.y;
    }

    pub fn zoom_center<V>(&mut self, ctx: &Context, factor: V) 
        where V: Into<Vector2<f32>>
    {
        let factor: Vector2<f32> = factor.into();
        let screen_rect = ctx.gfx.drawable_size();
        let screen_center = Point2 {
            x: screen_rect.0 / 2.0,
            y: screen_rect.1 / 2.0,
        };
        let world_center = self.screen_to_world_coords(screen_center);
        self.offset.x = world_center.x - (world_center.x - self.offset.x) / factor.x;
        self.offset.y = world_center.y - (world_center.y - self.offset.y) / factor.y;
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
        self.offset.x = world_center.x - (world_center.x - self.offset.x) / factor.x;
        self.offset.y = world_center.y - (world_center.y - self.offset.y) / factor.y;
        self.scale.x *= factor.x;
        self.scale.y *= factor.y;
    }
}
