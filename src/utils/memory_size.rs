use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemorySize(u64); // en bytes

impl MemorySize {
    pub fn new(bytes: u64) -> MemorySize {
        MemorySize(bytes)
    }

    pub fn set_bytes(&mut self, bytes: u64) {
        self.0 = bytes;
    }

    pub fn from_bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    pub fn bytes(self) -> u64 {
        self.0
    }

    pub fn kilobytes(self) -> f64 {
        self.0 as f64 / 1024.0
    }

    pub fn megabytes(self) -> f64 {
        self.0 as f64 / (1024.0 * 1024.0)
    }

    pub fn gigabytes(self) -> f64 {
        self.0 as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    pub fn bits(self) -> u64 {
        self.0 * 8
    }

    pub fn formatted_string(self) -> String {
        if self.0 >= 1024 * 1024 * 1024 {
            format!("{:.2} GB", self.gigabytes())
        } else if self.0 >= 1024 * 1024 {
            format!("{:.2} MB", self.megabytes())
        } else if self.0 >= 1024 {
            format!("{:.2} KB", self.kilobytes())
        } else {
            format!("{} B", self.bytes())
        }
    }

    pub fn print_memory_usage(&self) {
        println!("Memory Usage: {}", self.formatted_string());
    }
}
impl Add for MemorySize {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for MemorySize {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Mul<u64> for MemorySize {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self {
        Self(self.0 * rhs)
    }
}

impl Div<u64> for MemorySize {
    type Output = Self;

    fn div(self, rhs: u64) -> Self {
        Self(self.0 / rhs)
    }
}

impl fmt::Display for MemorySize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted_string())
    }
}
