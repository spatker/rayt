use std::ops::Neg;
use std::ops;

use rand::prelude::*;

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
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(&self) -> Vec3n {
        let v = self / self.len();
        Vec3n {x: v.x, y: v.y, z: v.z}
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x
        }
    }

    pub fn random(min: f32, max: f32) -> Self {
        let mut rng = thread_rng();
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
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

    // first paramater is the normal (N)
    pub fn reflect(&self, incoming: &Self) -> Self {
        Vec3n::from(
            Vec3::from(incoming) - (2.0* (self * incoming)) * self
        )
    }

    // first paramater is the normal (N)
    pub fn refract(&self, incoming: &Self, n: f32) -> Option<Self> {
        let cos_theta = f32::min(self * incoming, 1.0);
        let k = 1.0 - (n * n * (1.0 - (cos_theta * cos_theta)));
        if k > 0. {
            Some(Vec3n::from(n * incoming - (n * (self * incoming) + f32::sqrt(k)) * self))
        } else {
            None
        }
    }

    pub fn random_sphere() -> Self {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.len_squared() >= 1.0 {continue} else {return Vec3n::from(p)}
        }
    }

    pub fn random_disc() -> Self {
        loop {
            let mut p = Vec3::random(-1.0, 1.0);
            p.z = 0.;
            if p.len_squared() >= 1.0 {continue} else {return Vec3n::from(p)}
        }
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
        v.norm()
    }
}

impl Neg for Vec3n {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3n{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

}

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

impl_op_ex!(+ |a: &Vec3n, b: &Vec3n| -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

impl_op_ex_commutative!(+ |a: &Vec3n, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

impl_op_ex!(- |a: &Vec3n, b: &Vec3n| -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
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

impl_op_ex_commutative!(+ |a: &Vec3, b: f32| -> Vec3 {
    Vec3 {
        x: a.x + b,
        y: a.y + b,
        z: a.z + b,
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
