use std::ops::Neg;

/// 浮点数位模式的三个部分
#[derive(Debug, Clone, Copy)]
pub struct Parts {
    /// 符号
    pub sign: u64,
    /// 阶码
    pub exp: u64,
    /// 尾数
    pub mantissa: u64,
}

impl Parts {
    pub fn new(bits: u64) -> Self {
        let sign: u64 = bits >> 63 & 0b1;
        let exp: u64 = bits >> 52 & 0b111_1111_1111;
        let mantissa: u64 =
            bits & 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

        Self {
            sign,
            exp,
            mantissa,
        }
    }

    // 解码浮点数
    // (-1)^S * 2^E * M
    // TODO: 解码非常规浮点数
    pub fn decode(&self) -> f64 {
        /// 阶码偏移量
        pub const BIAS: u64 = 1023;

        let s = (-1.0_f64).powf(self.sign as f64);
        let e = 2.0_f64.powf(self.exp as f64 - BIAS as f64);

        let mut m = 1.0_f64;
        for i in 0..52 {
            let mask = 0b1 << i;
            let bit_at_i = self.mantissa & mask;
            if bit_at_i == 0 {
                continue;
            }
            let weight = 2.0_f64.powf(i as f64 - 52.0);
            m += weight;
        }

        s * e * m
    }

    pub fn bits(&self) -> u64 {
        (self.sign << 63) + (self.exp << 52) + self.mantissa
    }
}

impl Neg for Parts {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let sign = !self.sign << 63 >> 63;
        let exp = self.exp;
        let mantissa = self.mantissa;

        Self {
            sign,
            exp,
            mantissa,
        }
    }
}

#[cfg(test)]
mod parts_test {
    use super::Parts;
    use std::mem::transmute;

    #[test]
    fn test_parts_decode() {
        let floats: Vec<f64> = vec![
            0.0066, 0.0027, 0.0006, 0.04, 0.0037, 0.0125, 0.0078, 0.0048, 0.0016, 0.0065, 0.0132,
        ];
        for f in floats {
            let bits: u64 = f.to_bits();
            let parts = Parts::new(bits);
            let decoded = parts.decode();
            assert_eq!(decoded, f);
        }
    }

    #[test]
    fn test_parts_bits() {
        let floats: Vec<f64> = vec![
            0.0066, 0.0027, 0.0006, 0.04, 0.0037, 0.0125, 0.0078, 0.0048, 0.0016, 0.0065, 0.0132,
        ];
        for f in floats {
            let bits: u64 = f.to_bits();
            let parts = Parts::new(bits);

            assert_eq!(f.to_bits(), parts.bits())
        }
    }
}
