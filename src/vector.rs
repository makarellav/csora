pub const FOV_FACTOR: f32 = 1024.0;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn rotate_x(&self, angle: f32) -> Vec3 {
        let angle_radians = angle.to_radians();
        let sin = angle_radians.sin();
        let cos = angle_radians.cos();

        Vec3 {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.z * cos + self.y * sin,
        }
    }

    pub fn rotate_y(&self, angle: f32) -> Vec3 {
        let angle_radians = angle.to_radians();
        let sin = angle_radians.sin();
        let cos = angle_radians.cos();

        Vec3 {
            x: self.x * cos - self.z * sin,
            y: self.y,
            z: self.z * cos + self.x * sin,
        }
    }

    pub fn rotate_z(&self, angle: f32) -> Vec3 {
        let angle_radians = angle.to_radians();
        let sin = angle_radians.sin();
        let cos = angle_radians.cos();

        Vec3 {
            x: self.x * cos - self.y * sin,
            y: self.y * cos + self.x * sin,
            z: self.z,
        }
    }
}

pub fn project(vec3: &Vec3) -> Option<Vec2> {
    if vec3.z == 0.0 {
        None
    } else {
        Some(Vec2 {
            x: (vec3.x * FOV_FACTOR) / vec3.z,
            y: (vec3.y * FOV_FACTOR) / vec3.z,
        })
    }
}
