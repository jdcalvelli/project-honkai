pub fn range_map(val: f64, start1: f64, end1: f64, start2: f64, end2: f64) -> f64 {
    start2 + (end2 - start2) * ((val - start1) / (end1 - start1))
}

