#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub const fn new(position: Vec2, size: Vec2) -> Self {
        Self{ position, size }
    }
    pub const fn from_center(center: Vec2, size: Vec2) -> Self {
        Self::new(center - size / 2.0, size)
    }
    pub const fn from_xywh(x: f32, y: f32, width: f32, height: f32) -> Self {
        position : Vec2 = Vec2::new(x, y);
        size : Vec2 = Vec2::new(width, height);
        Self{ position, size }
    }

    pub fn left(self) -> f32 {
        self.position.x
    }
    pub fn right(self) -> f32 {
        self.position.x + self.size.x
    }
    pub fn bottom(self) -> f32 {
        self.position.y
    }
    pub fn top(self) -> f32 {
        self.position.y + self.size.y
    }

    pub fn center(self) -> Vec2 {
        self.position + self.size / 2.0
    }

    pub fn bottom_left(self) -> Vec2 {
        self.position
    }
    pub fn bottom_right(self) -> Vec2 {
        Vec2::new(
            self.position.x + self.size.x,
            self.position.y
        )
    }
    pub fn top_left(self) -> Vec2 {
        Vec2::new(
            self.position.x,
            self.position.y + self.size.y
        )
    }    
    pub fn top_right(self) -> Vec2 {
        self.position + self.size
    }


    pub fn width(self) -> f32 {
        self.size.x
    }
    pub fn height(self) -> f32 {
        self.size.y
    }

    pub fn is_empty(self) -> bool {
        self.size.x <= 0.0 || self.size.y <= 0.0
    }
    

    pub fn intersects(self, other: Rect) -> bool {
        self.left() < other.right() &&
        self.right() > other.left() &&
        self.bottom() < other.top() &&
        self.top() > other.bottom()
    }
    pub fn contains(self, other: Rect) -> bool {
        self.left() < other.left() &&
        self.right() > other.right() &&
        self.bottom() < other.bottom() &&
        self.top() > other.top()
    }
    pub fn contains_point(self, point: Vec2) -> bool {
        point.x > self.left() &&
        point.x < self.right() &&
        point.y > self.bottom() &&
        point.y < self.top()
    }
    pub fn area(self) -> f32 {
        self.size.x * self.size.y
    }

    pub fn translate(&mut self, amount: Vec2) {
        self.position += amount;
    }
    pub fn scale_size(&mut self, c: f32) {
        self.size *= c;
    }
    pub fn inflate(&mut self, amount: Vec2) {
        self.position -= amount;
        self.size += amount * 2.0;
    }
    pub fn inflate_from_center(&mut self, c: f32) {
        let original_center = self.center();
        self.inflate(c);
        self.position = original_center - self.size / 2.0;
    }
}