fn sum(x : Option<i32>, y : Option<i32>) -> Option<i32> {
    match (x, y) { // use tuple to avoid multiple matches
        (Some(x), Some(y)) => Some(x + y),
        _ => None
    }
}

fn sum2(x : Option<i32>, y : Option<i32>) -> Option<i32> {
    Some(x? + y?)
}

fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    match time_of_day {
        0..=21 => Some(5),
        22..=24 => Some(0),
        _ => None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(Some(1), Some(2)), Some(3));
        assert_eq!(sum(Some(1), None), None);
        assert_eq!(sum(None, Some(2)), None);
        assert_eq!(sum(None, None), None);
    }

    #[test]
    fn test_sum2() {
        assert_eq!(sum2(Some(1), Some(2)), Some(3));
        assert_eq!(sum2(Some(1), None), None);
        assert_eq!(sum2(None, Some(2)), None);
        assert_eq!(sum2(None, None), None);
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }
 
    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap_or(0), 5);
        let icecreams = maybe_icecream(22);
        assert_eq!(icecreams.unwrap_or(0), 0);
        let icecreams = maybe_icecream(25);
        assert_eq!(icecreams.unwrap_or_default(), 0);

    }
}
