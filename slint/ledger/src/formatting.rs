pub fn currency(cents: i64) -> String {
    let sign = if cents < 0 { "−" } else { "" };
    let absolute = cents.unsigned_abs();
    let dollars = absolute / 100;
    let fraction = absolute % 100;
    let digits = dollars.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, character) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(character);
    }
    format!("{sign}${grouped}.{fraction:02}")
}

pub fn whole_currency(cents: i64) -> String {
    currency(cents).trim_end_matches(".00").to_owned()
}

pub fn percent(basis_points: i32) -> String {
    let sign = if basis_points > 0 {
        "+"
    } else if basis_points < 0 {
        "−"
    } else {
        ""
    };
    let absolute = basis_points.unsigned_abs();
    format!("{sign}{}.{:02}%", absolute / 100, absolute % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_financial_values_deterministically() {
        assert_eq!(currency(128_442_052), "$1,284,420.52");
        assert_eq!(whole_currency(42_453_600), "$424,536");
        assert_eq!(percent(197), "+1.97%");
        assert_eq!(percent(-9), "−0.09%");
    }
}
