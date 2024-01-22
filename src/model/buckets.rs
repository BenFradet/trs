use rand::{distributions::Uniform, Rng};

pub struct Buckets {
    storage: Box<[u32]>,
    desired_size: usize,
}

// todo: find a way to implement rules on the generation
// e.g. I want to generate 4 elements but not more than one each
impl Buckets {
    // unsafe for large generation
    pub fn new<R: Rng + ?Sized>(
        r: &mut R,
        mut base_values: Box<[u32]>,
        desired_size: usize,
    ) -> Buckets {
        let missing_elements = desired_size as u32 - base_values.iter().sum::<u32>();
        let size = base_values.len();
        let distribution = Uniform::new(0, size);
        for _i in 0..missing_elements {
            let index = r.sample(distribution);
            base_values[index] = base_values[index] + 1;
        }
        Buckets {
            storage: base_values,
            desired_size,
        }
    }

    // should not consume self
    pub fn draw<R: Rng + ?Sized>(&self, r: &mut R) -> Vec<u32> {
        let mut res = Vec::new();
        let distribution = Uniform::new(0, self.storage.len());
        // clone to not mut self
        let mut st = self.storage.clone();
        for _i in 0..self.desired_size {
            loop {
                let sampled_idx = r.sample(distribution);
                let e = st[sampled_idx];
                if e > 0 {
                    res.push(sampled_idx as u32);
                    st[sampled_idx] = e - 1;
                    break;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use rand::rngs::OsRng;

    use super::*;

    #[test]
    // idempotent ignoring rng
    fn draw_should_be_idempotent() -> () {
        let mut rng = OsRng;
        let init: Box<[u32]> = Box::new([4, 2, 2, 2]);
        let desired_size = 16;
        let bs = Buckets::new(&mut rng, init.clone(), desired_size);
        let res1 = bs.draw(&mut rng);
        let res2 = bs.draw(&mut rng);
        assert_eq!(res1.len(), desired_size);
        assert_eq!(res2.len(), desired_size);
        assert_ne!(res1.iter().cmp(res2.iter()), Ordering::Equal);
        assert!(res1.iter().all(|a| a >= &0 && a < &(init.len() as u32)));
        assert!(res2.iter().all(|a| a >= &0 && a < &(init.len() as u32)));
    }

    #[test]
    fn draw_should_construct_vec_with_desired_size() -> () {
        let mut rng = OsRng;
        let init: Box<[u32]> = Box::new([4, 2, 2, 2]);
        let desired_size = 16;
        let bs = Buckets::new(&mut rng, init.clone(), desired_size);
        let res = bs.draw(&mut rng);
        assert!(res.iter().all(|a| a >= &0 && a < &(init.len() as u32)));
    }

    #[test]
    fn draw_should_construct_vec_between_0_and_len() -> () {
        let mut rng = OsRng;
        let init: Box<[u32]> = Box::new([4, 2, 2, 2]);
        let desired_size = 16;
        let bs = Buckets::new(&mut rng, init.clone(), desired_size);
        let res = bs.draw(&mut rng);
        assert_eq!(res.len(), desired_size);
    }

    #[test]
    fn new_copies_array_argument() -> () {
        let mut rng = OsRng;
        let init: Box<[u32]> = Box::new([4, 2, 2, 2]);
        let desired_size = 16;
        Buckets::new(&mut rng, init.clone(), desired_size);
        assert_eq!(init[0], 4);
        assert_eq!(init[1], 2);
        assert_eq!(init[2], 2);
        assert_eq!(init[3], 2);
    }

    #[test]
    fn new_generates_an_array_with_sum_desired_size() -> () {
        let mut rng = OsRng;
        let init: Box<[u32]> = Box::new([4, 2, 2, 2]);
        let desired_size = 16;
        let bs = Buckets::new(&mut rng, init, desired_size);
        assert_eq!(bs.storage.iter().sum::<u32>(), desired_size as u32);
    }

    #[test]
    fn new_has_same_length_as_init() -> () {
        let mut rng = OsRng;
        let init: Box<[u32]> = Box::new([4, 2, 2, 2]);
        let desired_size = 16;
        let bs = Buckets::new(&mut rng, init.clone(), desired_size);
        assert_eq!(bs.storage.len(), init.len());
    }

    #[test]
    fn new_adds_to_init() -> () {
        let mut rng = OsRng;
        let init: Box<[u32]> = Box::new([4, 2, 2, 2]);
        let desired_size = 16;
        let bs = Buckets::new(&mut rng, init.clone(), desired_size);
        assert!(bs.storage.iter().zip(init.iter()).all(|(a, b)| a >= b));
    }

    #[test]
    fn new_desired_size_is_arg() -> () {
        let mut rng = OsRng;
        let init: Box<[u32]> = Box::new([4, 2, 2, 2]);
        let desired_size = 16;
        let bs = Buckets::new(&mut rng, init, desired_size);
        assert_eq!(bs.desired_size, desired_size);
    }
}
