#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2 {
    pub start: Vec2,
    pub end: Vec2
}

impl Line2 {
    pub const fn new(start: Vec2, end: Vec2) -> Self {
        Line2{ start, end }
    }
    pub const fn from_direction(start: Vec2, dir: Vec2) -> Self {
        Line2{ start, start + dir }
    }
    pub const fn from_direction_and_length(start: Vec2, dir: Vec2, length: f32) -> Self {
        Line2{ start, start + dir * length}
    }

    pub fn length(self) -> f32 {
        self.start.distance(end)
    }
    pub fn length_squared(self) -> f32 {
        self.start.distance_squared(end)
    }

    pub fn direction(self) -> Vec2 {
        self.start - self.end
    }
    pub fn normalized_direction(self) -> Vec2 {
        self.direction.normalized()
    }

    pub fn center(self) -> Vec2 {
        (self.start + self.end) / 2.0
    }
    pub fn point_at(self, t: f32) -> Vec2 {
        self.start + self.direction * t
    }
}