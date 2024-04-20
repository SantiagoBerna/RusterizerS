use glam::UVec2;
use glam::Vec2;

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2
}

impl Line {
    pub fn new(start: Vec2, end:Vec2) -> Self {
        Self { start, end }
    }

    pub fn intersect(&self, line: &Line) -> Option<f32> {

        let x1 = self.start.x;
        let x2 = self.end.x;
        let x3 = line.start.x;
        let x4 = line.end.x;

        let y1 = self.start.y;
        let y2 = self.end.y;
        let y3 = line.start.y;
        let y4 = line.end.y;

        //let div = (self.start.x - self.end.x) * (line.start.y - line.end.y) - (self.start.y - self.end.y) * (line.start.x - line.end.x);
        let div = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4); 
        if div.abs() < std::f32::EPSILON { return None; }

        let num2 = (x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3);
        
        let u = -num2 / div;

        if u > 0.0 && u < 1.0 { Some(u) }
        else { None }
    }

    pub fn is_right(&self, point: Vec2) -> bool {
        (self.end.x - self.start.x) * (point.y - self.start.y) - (self.end.y - self.start.y) * (point.x - self.start.x) <= 0.0
    }

    pub fn is_left(&self, point: Vec2) -> bool {
        (self.end.x - self.start.x) * (point.y - self.start.y) - (self.end.y - self.start.y) * (point.x - self.start.x) >= 0.0
    }
}

#[derive(Clone, Copy, Debug, )]
pub struct BoundingBox {
    pub start: UVec2,
    pub end: UVec2
}

impl BoundingBox {

    pub fn new(start: UVec2, end: UVec2) -> Self {
        Self { start, end }
    }

    pub fn intersect(&self, other: &BoundingBox) -> Option<Self> {

        let x_intersect = self.start.x <= other.end.x && self.end.x >= other.start.x;
        let y_intersect = self.start.y <= other.end.y && self.end.y >= other.start.y;

        if x_intersect && y_intersect {
            Some(Self { 
                start: self.start.max(other.start),
                end: self.end.min(other.end) 
            })
        } else {
            None
        }
    }

    pub fn clip_line(&self, line: &Line) -> Option<Line> {

        let mut out_line = line.clone();

        let left_p = UVec2::new(self.start.x, self.end.y);
        let right_p = UVec2::new(self.end.x, self.start.y);

        let lines = [
            Line { start: self.start.as_vec2(), end: left_p.as_vec2() },
            Line { start: left_p.as_vec2(), end: self.end.as_vec2() },
            Line { start: self.end.as_vec2(), end: right_p.as_vec2() },
            Line { start: right_p.as_vec2(), end: self.start.as_vec2() }
        ];

        for box_line in &lines {

            let start_inside = box_line.is_right(out_line.start);
            let end_inside = box_line.is_right(out_line.end);

            if !start_inside && !end_inside {
                return None;
            }

            else if let Some(t) = box_line.intersect(&out_line) {
                let intersection = super::lerp(out_line.start, out_line.end, t);
    
                if start_inside && !end_inside {
                    out_line.end = intersection;
                } else if !start_inside && end_inside {
                    out_line.start = intersection;
                }
            }
        }

        Some(out_line)
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use super::{BoundingBox, Line};

    const A : BoundingBox = BoundingBox {
        start: glam::UVec2 { x: 100, y: 100 },
        end: glam::UVec2 { x: 200, y: 200 }
    };

    const B : BoundingBox = BoundingBox {
        start: glam::UVec2 { x: 50, y: 50 },
        end: glam::UVec2 { x: 150, y: 150 }
    };

    const C : BoundingBox = BoundingBox {
        start: glam::UVec2 { x: 200, y: 200 },
        end: glam::UVec2 { x: 300, y: 300 }
    };

    const D : BoundingBox = BoundingBox {
        start: glam::UVec2 { x: 0, y: 0 },
        end: glam::UVec2 { x: 639, y: 479 }
    };

    const L : Line = Line {
        start: Vec2 { x: -54.0, y: 431.0 },
        end: Vec2 { x: 257.0, y: -770.0 }
    };

    #[test]
    fn intersection() {

        let c = A.intersect(&B).unwrap();
        assert!(c.start.x == 100 && c.start.y == 100 && c.end.x == 150 && c.end.y == 150)
    }

    #[test]
    fn no_intersection() {
        let i = C.intersect(&B);
        assert!(i.is_none());
    }

    #[test]
    fn line_intersect() {
        let val = D.clip_line(&L);
        assert!(val.is_some());
    }
}