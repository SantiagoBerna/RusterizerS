use glam::UVec2;

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
}

#[cfg(test)]
mod tests {
    use super::BoundingBox;

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

    #[test]
    fn intersection() {

        let c = A.intersect(&B).unwrap();
        assert!(c.start.x == 100 && c.start.y == 100 && c.end.x == 150 && c.end.y == 150)
    }

    #[test]
    fn no_intersection() {
        let i = C.intersect(&B);
        assert!(i.is_none())
    }
}