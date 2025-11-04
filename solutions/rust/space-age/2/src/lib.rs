// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

impl Duration {
    pub fn into(self) -> u64 {
        self.0
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64; // years relative to Earth

    // One Earth year equals 365.25 Earth days, or 31,557,600 seconds.
    // If you were told someone was 1,000,000,000 seconds old, their age would be 31.69 Earth-years.
    fn years_during(d: &Duration) -> f64;
}

 macro_rules! impl_Planet {
    ($($planet:ty => $orbital_period:expr),+) => {
        $(impl Planet for $planet {
            const ORBITAL_PERIOD: f64 = $orbital_period;
            fn years_during(d: &Duration) -> f64 {
                let earth_seconds = 31_557_600.0;
                let duration_in_seconds = d.into() as f64;
                duration_in_seconds / (earth_seconds * $orbital_period)
            }
        })*
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl_Planet!(
    Mercury => 0.2408467,
    Venus   => 0.61519726,
    Earth   => 1.0,
    Mars    => 1.8808158,
    Jupiter => 11.862615,
    Saturn  => 29.447498,
    Uranus  => 84.016846,
    Neptune => 164.79132
);

/*
impl Planet for Mercury {}
impl Planet for Venus {}
impl Planet for Earth {}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}
*/