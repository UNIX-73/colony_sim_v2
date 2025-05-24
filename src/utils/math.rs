/// División con redondeo hacia abajo (como en coordenadas de mundos con chunks)
pub fn div_floor(a: i32, b: i32) -> i32 {
    let d = a / b;
    let r = a % b;
    if (r != 0) && ((r < 0) != (b < 0)) {
        d - 1
    } else {
        d
    }
}

/// Módulo matemático (siempre devuelve un valor positivo en el rango [0, b))
pub fn mod_floor(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

pub fn extract_decimals_64(value: f64) -> (i64, f64) {
    let int_part = value.floor();
    let decimals = value - int_part;
    (int_part as i64, decimals)
}

pub fn extract_decimals_32(value: f32) -> (i32, f32) {
    let int_part = value.floor();
    let decimals = value - int_part;
    (int_part as i32, decimals)
}
