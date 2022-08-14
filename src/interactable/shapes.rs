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
        let (sx, sy) = scale_to_pair(s);
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
        // TODO: Handle scaling
        // TODO: handle orientation
        let (sx, sy) = scale_to_pair(s);

        let dist = p.distance(tl);

        if dist > self.radius {
            return false;
        }
        if dist < self.radius * 3. / 4. {
            return true;
        }

        let p = p - tl;

        // Check against borders
        let py = p.y * 1.15470053838; // 2/sqrt(3)
        if py > self.radius || py < -self.radius {
            return false;
        }

        let px = 0.5 * py + p.x;
        if px > self.radius || px < -self.radius {
            return false;
        }

        if py - px > self.radius || py - px < -self.radius {
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
