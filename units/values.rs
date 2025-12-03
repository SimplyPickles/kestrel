use crate::units::metric_units::Unit;

pub struct KValue {
    pub unit: &'static Unit,
    pub value: f64,
}

impl KValue {
    pub fn find_exact(&self) -> f64 {
        self.value * self.unit.scalar
    }

    pub fn dim_analysis(&self, unit: Unit) -> f64 {
        self.find_exact() * unit.scalar
    }
}
