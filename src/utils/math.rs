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