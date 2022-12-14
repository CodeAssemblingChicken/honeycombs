use bevy::math::Vec2;

#[allow(dead_code)]
pub enum Shape {
    Quad(Quad),
    Circle(Circle),
    Hexagon(Hexagon),
}

impl ContainsPoint for Shape {
    fn contains_point(&self, p: Vec2, tl: Vec2, s: Option<Vec2>) -> bool {
        match self {
            Self::Quad(q) => q.contains_point(p, tl, s),
            Self::Circle(c) => c.contains_point(p, tl, s),
            Self::Hexagon(h) => h.contains_point(p, tl, s),
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::Quad(Quad {
            width: 1.,
            height: 1.,
        })
    }
}

pub struct Quad {
    pub width: f32,
    pub height: f32,
}

impl ContainsPoint for Quad {
    fn contains_point(&self, p: Vec2, tl: Vec2, s: Option<Vec2>) -> bool {
        let (sx, sy) = scale_to_pair(s);
        if p.x < tl.x - self.width * sx / 2. || p.x > tl.x + self.width * sx / 2. {
            return false;
        }
        if p.y < tl.y - self.height * sy / 2. || p.y > tl.y + self.height * sy / 2. {
            return false;
        }
        true
    }
}

pub struct Circle {
    pub radius: f32,
}

impl ContainsPoint for Circle {
    fn contains_point(&self, p: Vec2, tl: Vec2, s: Option<Vec2>) -> bool {
        // TODO: Handle non uniform scaling (sx != sy)
        let (sx, _sy) = scale_to_pair(s);
        // let c = tl
        //     - Vec2 {
        //         x: self.radius / 2.,
        //         y: self.radius / 2.,
        //     };
        return p.distance(tl) <= self.radius * sx;
    }
}

pub struct Hexagon {
    pub radius: f32,
    pub point_up: bool,
}

impl ContainsPoint for Hexagon {
    fn contains_point(&self, p: Vec2, tl: Vec2, s: Option<Vec2>) -> bool {
        // TODO: handle orientation
        // TODO: Handle non uniform scaling (sx != sy)
        let (sx, _sy) = scale_to_pair(s);

        let dist = p.distance(tl);
        let radius = sx * self.radius;

        if dist > radius {
            return false;
        }
        if dist < radius * 3. / 4. {
            return true;
        }

        let p = p - tl;

        // Check against borders
        let py = p.y * 1.15470053838; // 2/sqrt(3)
        if py > radius || py < -radius {
            return false;
        }

        let px = 0.5 * py + p.x;
        if px > radius || px < -radius {
            return false;
        }

        if py - px > radius || py - px < -radius {
            return false;
        }

        return true;
    }
}

pub trait ContainsPoint {
    fn contains_point(&self, point: Vec2, translation: Vec2, scale: Option<Vec2>) -> bool;
}

fn scale_to_pair(scale: Option<Vec2>) -> (f32, f32) {
    let mut sx = 1.0;
    let mut sy = 1.0;
    if let Some(s) = scale {
        sx = s.x;
        sy = s.y;
    }
    (sx, sy)
}
