mod units;

use units::metric_units::PREFIXES;
use units::values::KValue;

fn main() {
    for p in PREFIXES {
        let bob: KValue = KValue {
            unit: p,
            value: 9.0,
        };

        println!("{} {} {}", bob.unit.name, bob.value, bob.find_exact());
    }
}
