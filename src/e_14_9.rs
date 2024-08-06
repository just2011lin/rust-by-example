#![allow(dead_code)]

use std::marker::PhantomData;

struct Meter;

struct CentiMeter;

struct Length<Unit> {
    value: f64,
    unit: PhantomData<Unit>,
}

impl<Unit> std::ops::Add for Length<Unit> {
    type Output = Length<Unit>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            unit: PhantomData,
        }
    }
}

#[test]
fn main() {
    let a: Length<Meter> = Length {
        value: 13.2,
        unit: PhantomData,
    };
    let b: Length<Meter> = Length {
        value: 4.2,
        unit: PhantomData,
    };
    let c = a + b;
    assert_eq!(c.value, 17.4);

    let a: Length<CentiMeter> = Length {
        value: 13.2,
        unit: PhantomData,
    };
    let b: Length<CentiMeter> = Length {
        value: 4.2,
        unit: PhantomData,
    };
    let c = a + b;
    assert_eq!(c.value, 17.4);
}
