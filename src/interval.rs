#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn merge(a: Interval, b: Interval) -> Interval {
        Interval { min: a.min.min(b.min), max: a.max.max(b.max) }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: f64::MAX,
            max: f64::MIN,
        }
    }
}
