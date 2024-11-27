/// Iterator over points decoded from a polyline5 or polyline6.
/// See https://developers.google.com/maps/documentation/utilities/polylinealgorithm
pub struct PolylineIter<'a> {
    polyline: &'a [u8],
    lat: i32,
    lon: i32,
    scale: f64,
}

impl<'a> PolylineIter<'a> {
    pub fn new(polyline: &'a str, precision: u8) -> Self {
        assert!(precision <= 7, "i32 can hold up to 180 * 10^7");
        PolylineIter {
            polyline: polyline.as_bytes(),
            lat: 0,
            lon: 0,
            scale: 10.0_f64.powi(precision as i32),
        }
    }

    #[inline(always)]
    fn decode_segment(&mut self) -> Option<i32> {
        let mut result = 0;
        for (i, &byte) in self.polyline.iter().enumerate() {
            let chunk = (byte as i32) - 63;
            result |= (chunk & 0x1f) << i * 5;
            if chunk & 0x20 == 0 {
                self.polyline = &self.polyline[i + 1..];
                return Some(zigzag_decode(result as u32));
            }
        }
        None
    }
}

impl<'a> Iterator for PolylineIter<'a> {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let lat_change = self.decode_segment()?;
        let lon_change = self.decode_segment()?;
        self.lat += lat_change;
        self.lon += lon_change;
        let lat = self.lat as f64 / self.scale;
        let lon = self.lon as f64 / self.scale;
        return Some((lat, lon));
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // There are at least polyline.len() / 12 points as each i32 is encoded in 5 bits per char.
        // And at most polyline.len() / 2 points if each number (2 per point) is encoded only by a single char.
        let len = self.polyline.len();
        (len / 12, Some(len / 2))
    }
}

/// Zigzag encoded numbers store the sign in the least significant bit, which this function moves to the sign bit.
#[inline(always)]
fn zigzag_decode(i: u32) -> i32 {
    (i >> 1) as i32 ^ -((i & 1) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// Checks if the polyline contains only valid characters and ends with a complete point.
    fn check_polyline(polyline: &str) -> bool {
        let bytes = polyline.as_bytes();
        let mut stop_bytes = 0;
        for &byte in bytes {
            if (byte as u32) < 63 || (byte as u32) > 127 {
                return false;
            }
            if (byte - 63) & 0x20 == 0 {
                stop_bytes += 1;
            }
        }
        // The polyline is valid if it ends with a complete point.
        stop_bytes % 2 == 0 && bytes.last().map(|b| b & 0x20 == 0).unwrap_or(true)
    }

    #[test]
    fn zigzag() {
        assert_eq!(zigzag_decode(0), 0);
        assert_eq!(zigzag_decode(1), -1);
        assert_eq!(zigzag_decode(2), 1);
        assert_eq!(zigzag_decode(3), -2);
        assert_eq!(zigzag_decode(4), 2);
        assert_eq!(zigzag_decode(5), -3);
        assert_eq!(zigzag_decode(6), 3);
        assert_eq!(zigzag_decode(7), -4);
        assert_eq!(zigzag_decode(8), 4);
        assert_eq!(zigzag_decode(9), -5);
        assert_eq!(zigzag_decode(10), 5);
        assert_eq!(zigzag_decode(11), -6);
        assert_eq!(zigzag_decode(12), 6);
        assert_eq!(zigzag_decode(13), -7);
        assert_eq!(zigzag_decode(14), 7);
        assert_eq!(zigzag_decode(15), -8);
    }

    #[test]
    fn empty_polyline() {
        assert_eq!(PolylineIter::new("", 5).next(), None);
        assert_eq!(PolylineIter::new("", 6).next(), None);
        assert!(check_polyline(""));
    }

    #[test]
    fn single_point() {
        let polyline = "_p~iF~ps|U_";
        assert!(check_polyline(polyline));
        let mut iter = PolylineIter::new(polyline, 5);
        assert_eq!(iter.next(), Some((38.5, -120.2)));
        assert_eq!(iter.next(), None);

        let mut iter = PolylineIter::new(polyline, 6);
        assert_eq!(iter.next(), Some((3.85, -12.02)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn multiple_points() {
        let polyline = "angrIk~inAgwDybH_|D_{KeoEwtLozFo`Gre@tcA";
        assert!(check_polyline(polyline));
        let mut iter = PolylineIter::new(polyline, 5);
        assert_eq!(iter.next(), Some((55.58513, 12.99958)));
        assert_eq!(iter.next(), Some((55.61461, 13.04627)));
        assert_eq!(iter.next(), Some((55.64485, 13.11219)));
        assert_eq!(iter.next(), Some((55.67816, 13.18223)));
        assert_eq!(iter.next(), Some((55.71840, 13.22343)));
        assert_eq!(iter.next(), Some((55.71222, 13.21244)));
        assert_eq!(iter.next(), None);

        let mut iter = PolylineIter::new(polyline, 6);
        assert_eq!(iter.next(), Some((5.558513, 1.299958)));
        assert_eq!(iter.next(), Some((5.561461, 1.304627)));
        assert_eq!(iter.next(), Some((5.564485, 1.311219)));
        assert_eq!(iter.next(), Some((5.567816, 1.318223)));
        assert_eq!(iter.next(), Some((5.571840, 1.322343)));
        assert_eq!(iter.next(), Some((5.571222, 1.321244)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn broken_string() {
        // Last point is missing a lon change, so the whole points will be skipped.
        let polyline = "_p~iF~ps|U_ulLnnqC_mqNvxq";
        assert!(!check_polyline(polyline)); // the polyline is not valid, but still can be decoded.
        let mut iter = PolylineIter::new(polyline, 5);
        assert_eq!(iter.next(), Some((38.5, -120.2)));
        assert_eq!(iter.next(), Some((40.7, -120.95)));
        assert_eq!(iter.next(), None);

        let mut iter = PolylineIter::new(polyline, 6);
        assert_eq!(iter.next(), Some((3.85, -12.02)));
        assert_eq!(iter.next(), Some((4.07, -12.095)));
        assert_eq!(iter.next(), None);

        assert_eq!(iter.next(), None); // just to make sure it does not panic
    }

    #[test]
    fn invalid_symbols() {
        // `!` (33) is not a valid symbol for a polyline because it is not in the range [63, 127].
        let polyline = "!!!!";
        assert!(!check_polyline(polyline));
        let mut iter = PolylineIter::new(polyline, 5);
        assert_eq!(iter.next(), None);

        // Now let's add `!` in the middle of a valid polyline.
        let polyline = "_p~iF~ps|U_ulLnnqC!_mqNvxq";
        assert!(!check_polyline(polyline)); // the polyline is not valid, but still can be decoded.
        let mut iter = PolylineIter::new(polyline, 5);
        assert_eq!(iter.next(), Some((38.5, -120.2)));
        assert_eq!(iter.next(), Some((40.7, -120.95)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn size_hint() {
        let iter = PolylineIter::new("_p~iF~ps|U_ulLnnqC_mqNvxq`@", 5);
        // Size hint should not be precise as the number of points depends the distance between them.
        assert!(iter.size_hint().0 <= 3);
        assert!(iter.size_hint().1.unwrap() >= 3);
        assert_eq!(iter.count(), 3);
    }
}
