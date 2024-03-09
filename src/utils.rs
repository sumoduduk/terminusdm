pub fn create_range(num: u32, divisor: u32) -> Option<Vec<String>> {
    if divisor == 0 {
        return None;
    }

    let range_size = num / divisor;
    let mut ranges = Vec::new();

    let mut start = 0;

    for i in 0..divisor {
        let mut end = (start + range_size) - 1;
        if i == divisor - 1 {
            end = num
        }
        ranges.push(format!("bytes={}-{}", start, end));
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
            Some(vec![
                "bytes=0-5".to_string(),
                "bytes=6-11".to_string(),
                "bytes=12-17".to_string(),
                "bytes=18-25".to_string()
            ])
        );
    }

    #[test]
    fn test_create_range_normal_1() {
        assert_eq!(
            create_range(100, 10),
            Some(vec![
                "bytes=0-9".to_string(),
                "bytes=10-19".to_string(),
                "bytes=20-29".to_string(),
                "bytes=30-39".to_string(),
                "bytes=40-49".to_string(),
                "bytes=50-59".to_string(),
                "bytes=60-69".to_string(),
                "bytes=70-79".to_string(),
                "bytes=80-89".to_string(),
                "bytes=90-100".to_string()
            ])
        );
    }

    #[test]
    fn test_create_range_normal_2() {
        assert_eq!(
            create_range(50, 7),
            Some(vec![
                "bytes=0-6".to_string(),
                "bytes=7-13".to_string(),
                "bytes=14-20".to_string(),
                "bytes=21-27".to_string(),
                "bytes=28-34".to_string(),
                "bytes=35-41".to_string(),
                "bytes=42-50".to_string()
            ])
        );
    }

    #[test]
    fn test_create_range_normal_3() {
        assert_eq!(
            create_range(30, 5),
            Some(vec![
                "bytes=0-5".to_string(),
                "bytes=6-11".to_string(),
                "bytes=12-17".to_string(),
                "bytes=18-23".to_string(),
                "bytes=24-30".to_string()
            ])
        );
    }
    #[test]
    fn test_create_range_normal_4() {
        assert_eq!(
            create_range(1000, 8),
            Some(vec![
                "bytes=0-124".to_string(),
                "bytes=125-249".to_string(),
                "bytes=250-374".to_string(),
                "bytes=375-499".to_string(),
                "bytes=500-624".to_string(),
                "bytes=625-749".to_string(),
                "bytes=750-874".to_string(),
                "bytes=875-1000".to_string()
            ])
        );
    }

    #[test]
    fn test_create_range_normal_5() {
        assert_eq!(
            create_range(1500, 8),
            Some(vec![
                "bytes=0-186".to_string(),
                "bytes=187-373".to_string(),
                "bytes=374-560".to_string(),
                "bytes=561-747".to_string(),
                "bytes=748-934".to_string(),
                "bytes=935-1121".to_string(),
                "bytes=1122-1308".to_string(),
                "bytes=1309-1500".to_string()
            ])
        );
    }

    #[test]
    fn test_create_range_normal_6() {
        assert_eq!(
            create_range(2000, 8),
            Some(vec![
                "bytes=0-249".to_string(),
                "bytes=250-499".to_string(),
                "bytes=500-749".to_string(),
                "bytes=750-999".to_string(),
                "bytes=1000-1249".to_string(),
                "bytes=1250-1499".to_string(),
                "bytes=1500-1749".to_string(),
                "bytes=1750-2000".to_string()
            ])
        );
    }

    #[test]
    fn test_create_range_normal_7() {
        assert_eq!(
            create_range(24651, 8),
            Some(vec![
                "bytes=0-3080".to_string(),
                "bytes=3081-6161".to_string(),
                "bytes=6162-9242".to_string(),
                "bytes=9243-12323".to_string(),
                "bytes=12324-15404".to_string(),
                "bytes=15405-18485".to_string(),
                "bytes=18486-21566".to_string(),
                "bytes=21567-24651".to_string()
            ])
        );
    }
}

// #[test]
// fn test_create_range_divisor_zero() {
//     assert_eq!(create_range(25, 0), None);
// }
