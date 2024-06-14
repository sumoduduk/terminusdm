pub mod fs_utils;
pub mod to_vec;

pub fn create_range(num: u64, divisor: u64) -> Option<Vec<(u64, u64)>> {
    if divisor == 0 {
        return None;
    }

    let range_size = num / divisor;
    let mut ranges = Vec::new();

    let mut start = 0;

    for i in 0..divisor {
        let mut end = (start + range_size) - 1;
        if i == divisor - 1 {
            end = num - 1
        }
        ranges.push((start, end));
        start = end + 1;
    }

    Some(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_range_normal() {
        assert_eq!(
            create_range(25, 4),
            Some(vec![(0, 5), (6, 11), (12, 17), (18, 24),])
        );
    }

    #[test]
    fn test_create_range_normal_1() {
        assert_eq!(
            create_range(100, 10),
            Some(vec![
                (0, 9),
                (10, 19),
                (20, 29),
                (30, 39),
                (40, 49),
                (50, 59),
                (60, 69),
                (70, 79),
                (80, 89),
                (90, 99),
            ])
        );
    }

    #[test]
    fn test_create_range_normal_2() {
        assert_eq!(
            create_range(50, 7),
            Some(vec![
                (0, 6),
                (7, 13),
                (14, 20),
                (21, 27),
                (28, 34),
                (35, 41),
                (42, 49),
            ])
        );
    }

    #[test]
    fn test_create_range_normal_3() {
        assert_eq!(
            create_range(30, 5),
            Some(vec![(0, 5), (6, 11), (12, 17), (18, 23), (24, 29)])
        );
    }
    #[test]
    fn test_create_range_normal_4() {
        assert_eq!(
            create_range(1000, 8),
            Some(vec![
                (0, 124),
                (125, 249),
                (250, 374),
                (375, 499),
                (500, 624),
                (625, 749),
                (750, 874),
                (875, 999)
            ])
        );
    }

    #[test]
    fn test_create_range_normal_5() {
        assert_eq!(
            create_range(1500, 8),
            Some(vec![
                (0, 186),
                (187, 373),
                (374, 560),
                (561, 747),
                (748, 934),
                (935, 1121),
                (1122, 1308),
                (1309, 1499)
            ])
        );
    }

    #[test]
    fn test_create_range_normal_6() {
        assert_eq!(
            create_range(2000, 8),
            Some(vec![
                (0, 249),
                (250, 499),
                (500, 749),
                (750, 999),
                (1000, 1249),
                (1250, 1499),
                (1500, 1749),
                (1750, 1999)
            ])
        );
    }

    #[test]
    fn test_create_range_normal_7() {
        assert_eq!(
            create_range(24651, 8),
            Some(vec![
                (0, 3080),
                (3081, 6161),
                (6162, 9242),
                (9243, 12323),
                (12324, 15404),
                (15405, 18485),
                (18486, 21566),
                (21567, 24650)
            ])
        );
    }
}

// #[test]
// fn test_create_range_divisor_zero() {
//     assert_eq!(create_range(25, 0), None);
// }
