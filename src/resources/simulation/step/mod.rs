use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;

pub const STEP_UNIT_MS: f64 = 1.0;

/// Representa unidades de simulación basadas en milisegundos.
/// 1 StepUnit equivale a STEP_UNIT_MS milisegundos.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct StepUnit(pub u32);

impl StepUnit {
    /// Crea una nueva StepUnit a partir de milisegundos.
    pub fn from_ms(ms: u32) -> Self {
        StepUnit((ms as f64 / STEP_UNIT_MS) as u32)
    }

    /// Devuelve la cantidad de unidades internas (sin convertir).
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    /// Devuelve el equivalente en milisegundos.
    pub fn as_ms(&self) -> f64 {
        self.0 as f64 * STEP_UNIT_MS
    }

    /// Crea una StepUnit directamente desde unidades.
    pub fn from_units(units: u32) -> Self {
        StepUnit(units)
    }

    pub fn as_duration(&self) -> Duration {
        let total_ms = self.as_ms();
        Duration::from_secs_f64(total_ms / 1000.0)
    }
}

// Implementaciones de operadores

impl Add for StepUnit {
    type Output = StepUnit;

    fn add(self, other: StepUnit) -> StepUnit {
        StepUnit(self.0 + other.0)
    }
}

impl Sub for StepUnit {
    type Output = StepUnit;

    fn sub(self, other: StepUnit) -> StepUnit {
        StepUnit(self.0 - other.0)
    }
}

impl Mul<u32> for StepUnit {
    type Output = StepUnit;

    fn mul(self, rhs: u32) -> StepUnit {
        StepUnit(self.0 * rhs)
    }
}

impl Div<u32> for StepUnit {
    type Output = StepUnit;

    fn div(self, rhs: u32) -> StepUnit {
        StepUnit(self.0 / rhs)
    }
}

// Debug y Display

impl fmt::Debug for StepUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StepUnit({} units, {:.2} ms)", self.0, self.as_ms())
    }
}

impl fmt::Display for StepUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}ms", self.as_ms())
    }
}
