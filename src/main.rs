use std::fmt;
use std::ops;

trait Currency {
    type Value;
    fn to_normal(&self) -> f32;
    fn from_normal(f32) -> Self::Value;
    fn to<C: Currency>(&self) -> <C as Currency>::Value;
    fn from<C: Currency>(other: C) -> Self::Value;
}

macro_rules! currency {
    ($t:ident, $c:expr) => {
        #[derive(Copy, Clone)]
        struct $t(f32);

        impl Currency for $t {
            type Value = $t;

            fn to_normal(&self) -> f32 {
                self.0 * $c
            }

            fn from_normal(n: f32) -> Self::Value {
                $t(n / $c)
            }

            fn to<C: Currency>(&self) -> <C as Currency>::Value {
                C::from_normal(self.to_normal())
            }

            fn from<C: Currency>(other: C) -> Self::Value {
                Self::from_normal(other.to_normal())
            }
        }

        impl<C: Currency> ops::Add<C> for $t {
            type Output = <Self as Currency>::Value;

            fn add(self, rhs: C) -> Self::Output {
                Self::Output::from_normal(self.to_normal() + rhs.to_normal())
            }
        }

        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }
}

currency!(Credits, 1.0);
currency!(CBills, 97.1);

fn main() {
    let x = Credits(232.0);
    let y = CBills(25.0);

    println!("{}", x + y);
    println!("{}", y + x);
    println!("{}", x.to::<CBills>());
    println!("{}", y.to::<Credits>());
}
