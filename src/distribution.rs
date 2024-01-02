use rand::{distributions::OpenClosed01, Rng};

// geometric
#[derive(Debug, Copy, Clone)]
pub struct Distribution {
    p: f64,
}

impl Distribution {
    // valid for 0 >= p <= 1
    pub fn new(p: f64) -> Distribution {
        Distribution { p }
    }

    pub fn p(&self) -> f64 {
        self.p
    }

    pub fn sample<R: Rng + ?Sized>(&self, random: &mut R) -> u32 {
        let x: f64 = random.sample(OpenClosed01);
        x.log(1.0 - self.p).ceil() as u32
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;

    use super::*;

    #[test]
    fn sample_test() -> () {
        let d = Distribution::new(0.5);
        let mut r = OsRng;
        let res = d.sample(&mut r);
        // true 99.9% of the time
        assert!(res < 10)
    }
}
