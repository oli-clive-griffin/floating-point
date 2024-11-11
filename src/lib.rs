struct FloatingPointNumber {
    size: usize,
    bits: Vec<bool>,
    exponent_size: usize,
    bias: u32,
    // significand_size: usize,
}

impl FloatingPointNumber {
    fn new(bits: Vec<bool>, exponent_size: usize, significand_size: usize) -> Self {
        // 1 bit for the sign
        let size = 1 + exponent_size + significand_size;
        let bias = 2_u32.pow(exponent_size as u32 - 1) - 1;

        Self {
            size,
            bits,
            exponent_size,
            bias,
            // significand_size,
        }
    }

    /// renders using rust's floating point math, just for illustration
    pub fn as_f64(&self) -> f64 {
        let a = if self.sign() { -1.0 } else { 1.0 };
        let b = 2_f64.powi(self.exponent() as i32);
        let c = self.significand();

        a * b * c
    }

    fn sign(&self) -> bool {
        self.bits[0]
    }

    fn significand_bits(&self) -> &[bool] {
        &self.bits[self.exponent_size + 1..self.size]
    }

    fn exponent_bits(&self) -> &[bool] {
        &self.bits[1..self.exponent_size + 1]
    }

    fn exponent(&self) -> u32 {
        let exp_bits = self.exponent_bits();

        exp_bits.iter().fold(0, |acc, &bit| acc * 2 + bit as u32) - self.bias
    }

    fn significand(&self) -> f64 {
        let mut val = 1.0;
        for (i, &bit) in self.significand_bits().iter().enumerate() {
            if bit {
                val += 2.0_f64.powi(-((i + 1) as i32));
            }
        }
        val
    }
}

fn concat<T: Clone>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    a.clone().into_iter().chain(b.clone().into_iter()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_f64() {
        let bits = vec![false, true, false, false, true, true, true, false, true];

        // sign = 1

        // exponent:
        // 10011 = 19
        // bias = 2^(5-1) - 1 = 15
        // exponent = 19 - 15 = 4

        // significand:
        // 101 =
        // 1 // hidden bit
        // + (1 * (1/2))
        // + (0 * (1/4))
        // + (1 * (1/8))
        // = 1.625

        let exponent_size = 5;
        let significand_size = 3;
        let number = FloatingPointNumber::new(bits, exponent_size, significand_size);

        // 1 * 2^4 * 1.625 = 26
        assert_eq!(number.as_f64(), 26.0);
    }
}
