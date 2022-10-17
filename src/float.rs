use super::parts::Parts;

use core::cmp::PartialEq;
use std::fmt::{self, Debug, Display};
use std::ops::{Add, Neg, Sub};

/// 实现64位浮点数
///
/// ## 编码与解码：
/// * 浮点数编码位模式
/// * 位模式解码浮点数
///
/// ## 浮点运算
/// * 加
/// * 减
/// * 乘
/// * 除
#[derive(Debug, Clone, Copy)]
pub struct Float {
    float: f64,
    bits: u64,

    parts: Parts,
}

impl Float {
    pub fn new(float: f64) -> Self {
        let bits = float.to_bits();
        let parts = Parts::new(bits);

        Self { float, bits, parts }
    }

    pub fn f64(&self) -> f64 {
        self.float
    }
}

impl Add for Float {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (mut l, mut r) = (self.parts, other.parts);
        if l.exp < r.exp || (l.exp == r.exp && l.mantissa < r.mantissa) {
            std::mem::swap(&mut l, &mut r)
        }
        let (l, r) = (l, r);

        // 対阶
        let delta_exp = l.exp - r.exp;
        let mut output = Parts::new(0.0_f64.to_bits());
        output.sign = l.sign;
        output.exp = l.exp;

        // 尾数运算
        const DEFAULT_MANTISSA: u64 = 1 << 52;
        let mantissa_left = l.mantissa + DEFAULT_MANTISSA;
        // FIXME: 右移有精度损失
        let mantissa_right = (DEFAULT_MANTISSA + r.mantissa) >> delta_exp;
        let mut mantissa: u64 = mantissa_left + mantissa_right;
        if r.sign != l.sign {
            mantissa = mantissa_left - mantissa_right;
        }

        // 结果规格化
        while mantissa >= (DEFAULT_MANTISSA << 1) {
            // FIXME: 右移有精度损失
            mantissa >>= 1;
            output.exp += 1;
        }
        while mantissa < DEFAULT_MANTISSA {
            mantissa <<= 1;
            output.exp -= 1;
        }

        // 隐藏默认值
        output.mantissa = mantissa - DEFAULT_MANTISSA;

        let parts = output;
        let bits = parts.bits();
        let float: f64 = parts.decode();
        Self { float, bits, parts }
    }
}

impl Sub for Float {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.add(-other)
    }
}

impl Neg for Float {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let float: f64 = -f64::from_bits(self.bits);
        let bits = (!self.bits >> 63 << 63) + (self.bits << 1 >> 1);
        let parts = -self.parts;

        Self { float, bits, parts }
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

impl PartialEq<f64> for Float {
    fn eq(&self, other: &f64) -> bool {
        self.bits == other.to_bits()
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.float)
    }
}

#[cfg(test)]
mod test_float {
    use super::Float;

    #[test]
    fn test_add() {
        for (l, r) in [(0.0475, 0.0006), (0.1, 0.2), (-0.1, 0.2)] {
            let fl = Float::new(l);
            let fr = Float::new(r);

            println!("---\t---\t---\t---\t---\t---");
            println!("[f64]\t{}\t+\t{}\t=\t{}", l, r, l + r);
            println!("[Float]\t{}\t+\t{}\t=\t{}", fl, fr, fl + fr);

            assert_eq!(rounding((fl + fr).f64()), rounding(l + r))
        }
    }

    #[test]
    fn test_sub() {
        for (l, r) in [(0.0475, 0.0006), (0.1, 0.2), (-0.1, 0.2)] {
            let fl = Float::new(l);
            let fr = Float::new(r);

            println!("---\t---\t---\t---\t---\t---");
            println!("[f64]\t{}\t-\t{}\t=\t{}", l, r, l - r);
            println!("[Float]\t{}\t-\t{}\t=\t{}", fl, fr, fl + -fr);

            assert_eq!(rounding((fl - fr).f64()), rounding(l - r))
        }
    }

    fn rounding(f: f64) -> f64 {
        (f * 10000.0 + 0.5).floor() / 10000.0
    }
}
