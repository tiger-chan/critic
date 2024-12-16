#[rustfmt::skip]
fn k_factor(a: f32) -> f32 {
    if a > 2400.0 { 5.0 }
    else if a > 2200.0 { 10.0 }
    else if a > 2000.0 { 15.0 }
    else if a > 1800.0 { 20.0 }
    else if a > 1600.0 { 25.0 }
    else if a > 1400.0 { 30.0 }
    else if a > 1200.0 { 35.0 }
    else if a > 1000.0 { 40.0 }
    else if a > 800.0 { 60.0 }
    else { 80.0 }
}

pub fn calc_change(a: f32, b: f32, s: f32) -> (f32, f32) {
    let k = k_factor(a);
    let e = 1.0 / (1.0 + 10.0f32.powf((b - a) / 400.0));
    let c_a = k * (s - e);

    let k = k_factor(b);
    let e = 1.0 / (1.0 + 10.0f32.powf((a - b) / 400.0));
    let c_b = k * ((1.0 - s) - e);

    // Minimum values
    let c_a = if a < 100.0 && c_a < 0.0 { 0.0 } else { c_a };
    let c_b = if b < 100.0 && c_b < 0.0 { 0.0 } else { c_b };

    (c_a.round(), c_b.round())
}
