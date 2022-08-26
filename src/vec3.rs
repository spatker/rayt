use std::ops;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3n {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn norm(&self) -> Self {
        self / self.len()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x
        }
    }
}

impl Vec3n {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3n::from(&Vec3{x, y, z})
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3n::from(
            &Vec3::from(self).cross(&Vec3::from(rhs))
        )
    }
}

impl From<f32> for Vec3 {
    fn from(a: f32) -> Self {
        Vec3{x: a, y: a, z: a}
    }
}

impl From<&Vec3n> for Vec3 {
    fn from(v: &Vec3n) -> Self {
        Vec3{x: v.x, y: v.y, z: v.z}
    }
}

impl From<Vec3n> for Vec3 {
    fn from(v: Vec3n) -> Self {
        Vec3{x: v.x, y: v.y, z: v.z}
    }
}

impl From<&Vec3> for Vec3n {
    fn from(v: &Vec3) -> Self {
        let n = v.norm();
        Vec3n {x: n.x, y: n.y, z: n.z}
    }
}

impl From<Vec3> for Vec3n {
    fn from(v: Vec3) -> Self {
        let n = v.norm();
        Vec3n {x: n.x, y: n.y, z: n.z}
    }
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

impl_op_ex!(* |a: &Vec3, b: &Vec3| -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z 
});

impl_op_ex!(* |a: &Vec3n, b: &Vec3n| -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z 
});

impl_op_ex_commutative!(* |a: &Vec3n, b: &Vec3| -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z 
});

impl_op_ex_commutative!(* |a: &Vec3, b: f32| -> Vec3 {
    Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});

impl_op_ex_commutative!(* |a: &Vec3n, b: f32| -> Vec3 {
    Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});

impl_op_ex!(/ |a: &Vec3, b: f32| -> Vec3 {
    Vec3 {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    }
});
