pub mod math {
    pub fn round_to_significant_digits(num: f64, digits: i32) -> f64 {
        // println!("{num}");
        if num == 0.0 {
            return 0.0; // Handling special case for 0
        }

        let magnitude = (num.abs().log10().floor() + 1.0) as i32;
        let scale = 10_f64.powi(digits - magnitude);
        (num * scale).round() / scale
    }

    pub fn round_up_to_n_decimal(num: f64, n: i32) -> f64 {
        (num * f64::powf(10.0, n as f64)).round() / f64::powf(10.0, n as f64)
    }

    pub fn round_up_to_nearest_5<T>(n: T) -> i32 
    where T: Into<f64>,
    {
        let n = n.into();
        let remainder = n % 5.0;
        if remainder == 0.0 {
            n as i32
        } else {
            (n - remainder + 5.0) as i32
        }
    }

}
