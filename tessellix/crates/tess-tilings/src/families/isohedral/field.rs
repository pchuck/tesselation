//! Exact quadratic-field arithmetic for the isohedral family: elements of
//! `K = Q(√3)` in canonical form `re + sq·√3` with `re, sq ∈ ℚ`.
//!
//! Rationals are the subfield `sq = 0`, so every instance type (whether its
//! Tactile coefficients are rational or `√3`-decimal) shares one arithmetic.
//! Canonical form is unique (`Rat` normalizes sign/denominator/gcd, and the
//! `(re, sq)` pair is the unique reduced `a + b√3`), which is what makes the
//! shared-edge bit-identity argument work: equal exact values are equal
//! `(re, sq)` pairs, hence one canonical `f64` rounding per coordinate
//! (docs/isohedral-generator-design.md §2).

use std::ops::{Add, AddAssign, Mul, Neg, Sub};

/// An exact rational `n/d` with `d > 0` and `gcd(|n|, d) = 1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rat {
    n: i128,
    d: i128,
}

impl Rat {
    /// Normalizing constructor (const: Euclid loop; denominators stay
    /// far below `i128` overflow in this family — design doc §2.1).
    pub const fn new(n: i128, d: i128) -> Self {
        assert!(d > 0, "Rat denominator must be positive");
        let (mut n, mut d) = (n, d);
        // gcd via Euclid (|n| may be 0: gcd(0, d) = d → 0/1)
        let (mut a, mut b) = (if n < 0 { -n } else { n }, d);
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        if a > 1 {
            n /= a;
            d /= a;
        }
        Self { n, d }
    }

    pub const fn zero() -> Self {
        Self { n: 0, d: 1 }
    }

    pub const fn one() -> Self {
        Self { n: 1, d: 1 }
    }

    /// Test-battery support: the pairing-D and transcription checks read
    /// these through the exact-geometry probes; production code does not.
    #[cfg_attr(not(test), allow(dead_code))]
    pub const fn is_zero(self) -> bool {
        self.n == 0
    }

    pub const fn add(self, o: Self) -> Self {
        // n1/d1 + n2/d2 = (n1·d2 + n2·d1)/(d1·d2)
        Self::new(self.n * o.d + o.n * self.d, self.d * o.d)
    }

    pub const fn sub(self, o: Self) -> Self {
        Self::new(self.n * o.d - o.n * self.d, self.d * o.d)
    }

    pub const fn mul(self, o: Self) -> Self {
        Self::new(self.n * o.n, self.d * o.d)
    }

    pub const fn neg(self) -> Self {
        Self {
            n: -self.n,
            d: self.d,
        }
    }

    /// Test-battery support: inversion enters only the pairing-D census
    /// (`D⁻¹`); production generation never inverts.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn inv(self) -> Option<Self> {
        if self.n == 0 {
            return None;
        }
        let (n, d) = (self.n, self.d);
        Some(if n > 0 {
            Self { n: d, d: n }
        } else {
            Self { n: -d, d: -n }
        })
    }

    /// The canonical float: one rounding of the exact value.
    pub fn to_f64(self) -> f64 {
        self.n as f64 / self.d as f64
    }
}

impl Add for Rat {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Rat::add(self, o)
    }
}

impl Sub for Rat {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Rat::sub(self, o)
    }
}

impl Mul for Rat {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        Rat::mul(self, o)
    }
}

impl Neg for Rat {
    type Output = Self;
    fn neg(self) -> Self {
        Rat::neg(self)
    }
}

/// An element of `Q(√3)`: `re + sq·√3`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Q {
    re: Rat,
    sq: Rat,
}

impl Q {
    /// A rational element (`sq = 0`).
    pub const fn r(n: i128, d: i128) -> Self {
        Self {
            re: Rat::new(n, d),
            sq: Rat::zero(),
        }
    }

    /// A general element `a + b√3`.
    pub const fn q(an: i128, ad: i128, bn: i128, bd: i128) -> Self {
        Self {
            re: Rat::new(an, ad),
            sq: Rat::new(bn, bd),
        }
    }

    pub const fn zero() -> Self {
        Self {
            re: Rat::zero(),
            sq: Rat::zero(),
        }
    }

    pub const fn one() -> Self {
        Self {
            re: Rat::one(),
            sq: Rat::zero(),
        }
    }

    /// Test-battery support (the IH7 transcription check asserts its
    /// record genuinely leaves the rational subfield).
    #[cfg_attr(not(test), allow(dead_code))]
    pub const fn is_rational(self) -> bool {
        self.sq.is_zero()
    }

    pub const fn add(self, o: Self) -> Self {
        Self {
            re: self.re.add(o.re),
            sq: self.sq.add(o.sq),
        }
    }

    pub const fn sub(self, o: Self) -> Self {
        Self {
            re: self.re.sub(o.re),
            sq: self.sq.sub(o.sq),
        }
    }

    pub const fn neg(self) -> Self {
        Self {
            re: self.re.neg(),
            sq: self.sq.neg(),
        }
    }

    /// `(a + b√3)(c + d√3) = (ac + 3bd) + (ad + bc)√3`.
    pub const fn mul(self, o: Self) -> Self {
        let ac = self.re.mul(o.re);
        let three_bd = self
            .sq
            .mul(o.sq)
            .add(self.sq.mul(o.sq))
            .add(self.sq.mul(o.sq));
        Self {
            re: ac.add(three_bd),
            sq: self.re.mul(o.sq).add(self.sq.mul(o.re)),
        }
    }

    /// Conjugation inverse: `1/(a + b√3) = (a − b√3)/(a² − 3b²)`.
    /// Test-battery support: the pairing-D census inverts the left
    /// pairing (`QAff::inverse`); generation never does.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn inv(self) -> Option<Self> {
        let den = self
            .re
            .mul(self.re)
            .sub(self.sq.mul(self.sq).mul(Q::r(3, 1).into_rat()));
        den.inv().and_then(|di| {
            Some(Self {
                re: self.re.mul(di),
                sq: self.sq.mul(di).neg(),
            })
        })
    }

    fn into_rat(self) -> Rat {
        self.re
    }

    /// The canonical float emission: `fl(a) + fl(b)·fl(√3)` in fixed order.
    /// Correctly-rounded `3f64.sqrt()` keeps this deterministic, and equal
    /// field elements are equal `(re, sq)` pairs, so equal values emit
    /// bit-identical floats (design doc §2).
    pub fn to_f64(self) -> f64 {
        self.re.to_f64() + self.sq.to_f64() * 3.0f64.sqrt()
    }
}

impl Add for Q {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Q::add(self, o)
    }
}

impl AddAssign for Q {
    fn add_assign(&mut self, o: Self) {
        *self = Q::add(*self, o);
    }
}

impl Sub for Q {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Q::sub(self, o)
    }
}

impl Mul for Q {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        Q::mul(self, o)
    }
}

impl Neg for Q {
    type Output = Self;
    fn neg(self) -> Self {
        Q::neg(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rat_normalizes_and_compares() {
        assert_eq!(Rat::new(2, 4), Rat::new(1, 2));
        assert_eq!(Rat::new(-3, 9), Rat::new(-1, 3));
        assert_eq!(Rat::new(0, 7), Rat::zero());
        assert_eq!(Rat::new(6, 3), Rat::new(2, 1));
    }

    #[test]
    fn rat_ops() {
        assert_eq!(Rat::new(1, 3) + Rat::new(1, 6), Rat::new(1, 2));
        assert_eq!(Rat::new(1, 3) - Rat::new(1, 6), Rat::new(1, 6));
        assert_eq!(Rat::new(2, 3) * Rat::new(3, 4), Rat::new(1, 2));
        assert_eq!(Rat::new(3, 4).inv(), Some(Rat::new(4, 3)));
        assert_eq!(Rat::zero().inv(), None);
    }

    #[test]
    fn q_field_ops() {
        let s3 = Q::q(0, 1, 1, 1); // √3
        assert_eq!(s3 * s3, Q::r(3, 1));
        assert_eq!(s3 * s3 * s3, Q::q(0, 1, 3, 1));
        assert_eq!(s3.inv(), Some(Q::q(0, 1, 1, 3))); // 1/√3 = √3/3
        let x = Q::q(3, 2, -1, 4);
        let xi = x.inv().expect("x is a field unit");
        assert_eq!(x * xi, Q::one());
        // rationals stay rational under +,-,*
        assert!(Q::r(1, 2).mul(Q::r(2, 3)).is_rational());
        assert!(Q::r(1, 2).add(Q::r(2, 3)).is_rational());
    }

    #[test]
    fn q_to_f64_canonical() {
        // equal values reached by different paths emit the same float
        let a = Q::r(137, 200);
        let b = Q::r(117, 200) + Q::r(1, 10);
        assert_eq!(a, b);
        assert_eq!(a.to_f64().to_bits(), b.to_f64().to_bits());
        // √3/6 via two paths: direct vs (1/2)·(√3/3)
        let c = Q::q(0, 1, 1, 6);
        let d = Q::r(1, 2) * Q::q(0, 1, 1, 3);
        assert_eq!(c, d);
        assert_eq!(c.to_f64().to_bits(), d.to_f64().to_bits());
    }
}
