///
/// A range of values or a single value.
///
#[derive(Debug)]
pub enum RangeOrValue<T> {
    /// A range of values. both the start value and end value are inclusive
    Range(T, T),

    /// A single value.
    Value(T),
}

impl<T> RangeOrValue<T> {
    /// Returns `true` if the value is a range, not a single value.
    pub fn is_range(&self) -> bool {
        matches!(self, RangeOrValue::Range(_, _))
    }

    /// Returns `true` if the value is a single value, not a range.
    pub fn is_value(&self) -> bool {
        matches!(self, RangeOrValue::Value(_))
    }
}

impl<T: PartialEq> PartialEq for RangeOrValue<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RangeOrValue::Range(start1, end1), RangeOrValue::Range(start2, end2)) => {
                start1 == start2 && end1 == end2
            }
            (RangeOrValue::Value(val1), RangeOrValue::Value(val2)) => val1 == val2,
            _ => false,
        }
    }
}

impl<T: PartialOrd> RangeOrValue<T> {
    /// Check if the value is contained in the range or is equal to the value.
    pub fn contains(&self, value: &T) -> bool {
        match self {
            RangeOrValue::Range(start, end) => value >= start && value <= end,
            RangeOrValue::Value(val) => val == value,
        }
    }
}

impl<T: std::str::FromStr + PartialOrd> std::str::FromStr for RangeOrValue<T> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse() {
            Ok(RangeOrValue::Value(value))
        } else {
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() != 2 {
                return Err("Invalid range format");
            }
            let start = parts[0].parse().map_err(|_| "Invalid range start value")?;
            let end = parts[1].parse().map_err(|_| "Invalid range end value")?;
            if start >= end {
                return Err("Invalid range: start value must be less than end value");
            }
            Ok(RangeOrValue::Range(start, end))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_from_str_single_value() {
        let value = "3".parse::<RangeOrValue<i32>>().unwrap();
        assert!(value.is_value());
        assert_eq!(value, RangeOrValue::Value(3));
    }

    #[test]
    fn test_range_from_str_range() {
        let value = "3-5".parse::<RangeOrValue<i32>>().unwrap();
        assert!(value.is_range());
        assert_eq!(value, RangeOrValue::Range(3, 5));
    }

    #[test]
    fn test_range_contains() {
        let value = "3-5".parse::<RangeOrValue<i32>>().unwrap();
        assert!(value.is_range());
        assert!(value.contains(&3));
        assert!(value.contains(&4));
        assert!(value.contains(&5));
        assert!(!value.contains(&2));
        assert!(!value.contains(&6));

        let value = "9".parse::<RangeOrValue<i32>>().unwrap();
        assert!(value.is_value());
        assert!(value.contains(&9));
        assert!(!value.contains(&2));
        assert!(!value.contains(&6));
    }

    #[test]
    fn test_range_or_value_from_str_invalid_range() {
        assert_eq!(
            "9-2".parse::<RangeOrValue<i32>>(),
            Err("Invalid range: start value must be less than end value")
        );
    }
}
