pub mod math {
    pub fn round_to_significant_digits(num: f64, digits: i32) -> f64 {
        // println!("{num}");
        if num == 0.0 {
            return 0.0 // Handling special case for 0
        }

        let magnitude = (num.abs().log10().floor() + 1.0) as i32;
        let scale = 10_f64.powi(digits - magnitude);
        (num * scale).round() / scale
    }

    pub fn round_decimal_places(num: f64, digits: i32, decimal_places: i32) -> f64 {
        let dec_places = f64::powf(10.0, decimal_places as f64);
        let num = round_to_significant_digits(num, digits);

        if (num * dec_places) % 1.0 != 0.0 {
            let rounded_num = (num * dec_places).round() / dec_places;
            rounded_num
        } else {
            num
        }
    }

    pub fn round_up_to_first_decimal(num: f64) -> f64 {
        (num * 10.0).round() / 10.0
    }

}