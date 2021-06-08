use once_cell::sync::Lazy;
use rand::{Rng, SeedableRng, StdRng};
use std::mem::size_of;
use std::slice;
use std::sync::Mutex;
use std::usize;

static INSTANCE: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(StdRng::new().unwrap()));

pub fn random() -> f64 {
    INSTANCE.lock().unwrap().gen::<f64>()
}

unsafe fn usize_from_u8(s8: &[u8]) -> &[usize] {
    slice::from_raw_parts(s8.as_ptr() as *const usize, s8.len() / size_of::<usize>())
}

pub fn reseed_from_f64(seed: f64) {
    unsafe {
        let seed_be_bytes = seed.to_be_bytes();
        let seed_bytes: &[usize] = usize_from_u8(&seed_be_bytes);
        INSTANCE.lock().unwrap().reseed(seed_bytes)
    }
}
