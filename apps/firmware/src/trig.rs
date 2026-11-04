pub fn sin_deg(deg: i32) -> i32 {
    let d = deg.rem_euclid(360);
    match d {
        0..=90 => sin_quarter(d),
        91..=180 => sin_quarter(180 - d),
        181..=270 => -sin_quarter(d - 180),
        _ => -sin_quarter(360 - d),
    }
}

pub fn cos_deg(deg: i32) -> i32 {
    sin_deg(deg + 90)
}

fn sin_quarter(d: i32) -> i32 {
    const TABLE: [i32; 7] = [0, 259, 500, 707, 866, 966, 1000];
    const ANGLES: [i32; 7] = [0, 15, 30, 45, 60, 75, 90];

    for i in 0..6 {
        if d >= ANGLES[i] && d <= ANGLES[i + 1] {
            let span = ANGLES[i + 1] - ANGLES[i];
            let frac = d - ANGLES[i];
            let val_span = TABLE[i + 1] - TABLE[i];
            return TABLE[i] + (val_span * frac) / span;
        }
    }
    1000
}
