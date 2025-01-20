use rstu::core::{Time,Timebase};

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_time_creation() {
        let time = Time::new(1000);
        assert_eq!(time.to_millisecond(), 1000);

        let time = Time::from_millisecond(2000);
        assert_eq!(time.to_millisecond(), 2000);

        let time = Time::from_seconds(&2.5);
        assert_eq!(time.to_millisecond(), 2500);
    }

    #[test]
    fn test_time_operations() {
        let time1 = Time::new(1000);
        let time2 = Time::new(2000);

        let sum = time1 + time2;
        assert_eq!(sum.to_millisecond(), 3000);

        let difference = time2 - time1;
        assert_eq!(difference.to_millisecond(), 1000);

        let product = time1 * 2.0;
        assert_eq!(product.to_millisecond(), 2000);

        let quotient = time2 / 2.0;
        assert_eq!(quotient.to_millisecond(), 1000);
    }

    #[test]
    fn test_timecode_conversion() {
        let timebase = Timebase{fps:30,drop_frame:false};
        let time = Time::from_timecode("00:00:05:15", &timebase).unwrap();
        assert_eq!(time.to_millisecond(), 5150);

        let timecode = time.to_timecode(&timebase);
        assert_eq!(timecode, "00:00:05:15");
    }

    #[test]
    fn test_timestamp_conversion() {
        let time = Time::from_timestamp("00:00:05:150").unwrap();
        assert_eq!(time.to_millisecond(), 5150);

        let timestamp = time.to_timestamp();
        assert_eq!(timestamp, "00:00:05:150");
    }

    #[test]
    fn test_time_comparison() {
        let time1 = Time::new(1000);
        let time2 = Time::new(2000);

        assert!(time1 < time2);
        assert!(time2 > time1);
        assert!(time1 <= time2);
        assert!(time2 >= time1);
        assert_eq!(time1, Time::new(1000));
        assert_ne!(time1, time2);
    }
}
