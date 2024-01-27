use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct MapRange {
    pub source: i64,
    pub destination: i64,
    pub range: i64,
}

impl MapRange {
    pub fn from(data: &str) -> MapRange {
        let split_str: Vec<&str> = data.split(" ").collect();
        MapRange {
            source: split_str[1].parse().unwrap(),
            destination: split_str[0].parse().unwrap(),
            range: split_str[2].parse().unwrap(),
        }
    }

    pub fn cmp(&self, source: i64) -> Ordering {
        if source - self.source < 0 {
            return Ordering::Greater;
        } else if source - self.source >= self.range {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_works_as_expected() {
        let expected = MapRange {
            source: 98,
            destination: 50,
            range: 2,
        };
        let actual = MapRange::from("50 98 2");
        assert_eq!(actual, expected);
    }

    #[test]
    fn maprange_cmp_works_as_expected() {
        let mr = MapRange {
            source: 98,
            destination: 50,
            range: 2,
        };

        let eq = mr.cmp(99);
        let lt = mr.cmp(100);
        let gt = mr.cmp(97);

        assert_eq!(eq, Ordering::Equal);
        assert_eq!(lt, Ordering::Less);
        assert_eq!(gt, Ordering::Greater);
    }
}
