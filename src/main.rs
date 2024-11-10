struct FloatingPointNumber {
    size: usize,
    bits: Vec<bool>,
    exponent_size: usize,
    // significand_size: usize,
}

impl FloatingPointNumber {
    fn new(bits: Vec<bool>, exponent_size: usize, significand_size: usize) -> Self {
        // 1 bit for the sign
        let size = 1 + exponent_size + significand_size;

        Self {
            size,
            bits,
            exponent_size,
            // significand_size,
        }
    }

    /// renders using rust's floating point math, just for illustration
    pub fn as_f64(&self) -> f64 {
        let a = if self.sign() { -1.0 } else { 1.0 };

        let exp = self.exponent();
        let b = 2_f64.powi(exp as i32);

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

        to_u32(exp_bits)
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

fn to_u32(bits: &[bool]) -> u32 {
    bits.iter().fold(0, |acc, &bit| acc * 2 + bit as u32)
}

fn concat<T: Clone>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    a.clone().into_iter().chain(b.clone().into_iter()).collect()
}

fn main() {
    let sign = vec![false]; // positive
    let exponent = vec![false, false, true, true, true]; // 7
    let significand = vec![true, false, true]; // interpreted as 1.625 (1 + (1/2) + (1/8))

    // value should be 1 * 2^7 * 1.625 = 208

    let number = FloatingPointNumber::new(
        concat(&sign, &concat(&exponent, &significand)),
        exponent.len(),
        significand.len(),
    );

    println!("{}", number.render());
}
