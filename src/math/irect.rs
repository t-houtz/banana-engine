#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IRect {
    pub position: IVec2,
    pub size: IVec2
}

impl IRect {
    pub const fn new(position: IVec2, size: IVec2) -> Self {
        Self{ position, size }
    }
    pub const fn from_xywh(x: i32, y: i32, w: i32, h: i32) -> Self {
        let position: IVec2 = IVec2::new(x, y);
        let size: IVec2 = IVec2::new(w, h);
        Self{ position, size }
    }

    pub fn left(&self) -> i32 {
        self.position.x
    }
    pub fn right(&self) -> i32 {
        self.position.x + self.size.x
    }
    pub fn bottom(&self) -> i32 {
        self.position.y
    }
    pub fn top(&self) -> i32 {
        self.position.y + self.size.y
    }

    pub fn bottom_left(&self) -> IVec2 {
        self.position
    }
    pub fn bottom_right(&self) -> IVec2 {
        IVec2::new(
            self.position.x + self.size.x,
            self.position.y
        )
    }
    pub fn top_left(&self) -> IVec2 {
        IVec2::new(
            self.position.x,
            self.position.y + self.size.y
        )
    }
    pub fn top_right(&self) -> IVec2 {
        self.position + self.size
    }

    pub fn width(&self) -> i32 {
        self.size.x
    }
    pub fn height(&self) -> i32 {
        self.size.y
    }

    pub fn is_empty(&self) -> bool {
        self.size.x <= 0 || self.size.y <= 0
    }
    

    pub fn intersects(self, other: IRect) -> bool {
        self.left() < other.right() &&
        self.right() > other.left() &&
        self.bottom() < other.top() &&
        self.top() > other.bottom()
    }
    pub fn contains(self, other: IRect) -> bool {
        self.left() <= other.left() &&
        self.right() > other.right() &&
        self.bottom() <= other.bottom() &&
        self.top() > other.top()
    }
    pub fn contains_point(self, point: IVec2) -> bool {
        point.x >= self.left() &&
        point.x < self.right() &&
        point.y >= self.bottom() &&
        point.y < self.top()
    }
    pub fn area(self) -> i32 {
        self.size.x * self.size.y
    }

    pub fn translate(&mut self, amount: IVec2) {
        self.position += amount;
    }
    pub fn scale_size(&mut self, c: i32) {
        self.size *= c;
    }
    pub fn inflate(&mut self, amount: IVec2) {
        self.position -= amount;
        self.size += amount * 2;
    }
}