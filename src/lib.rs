/// Iterator over points, decoded from a polyline5 or polyline6.
/// See https://developers.google.com/maps/documentation/utilities/polylinealgorithm
///
/// ```
/// use polyline_iter::PolylineIter;
/// let iter = PolylineIter::new(6, "avs_iB}xlxWissBw|zEu``AsxgCyoaAm_z@");
/// assert_eq!(
///     iter.collect::<Vec<_>>(),
///     vec![
///         (55.585137, 12.999583),
///         (55.644854, 13.112187),
///         (55.678161, 13.182229),
///         (55.712222, 13.212444),
///     ]
/// );
///
/// // If the points are not needed, the iterator can be used directly
/// assert_eq!(PolylineIter::new(5, "avs_iB}xlxWissBw|zEu``AsxgCyoaAm_z@").count(), 4);
///
/// // Iterator approach allows to transcode polyline to another precision without intermediate allocations.
/// let polyline5 = polyline_iter::encode(5, PolylineIter::new(6, "avs_iB}xlxWissBw|zEu``AsxgCyoaAm_z@"));
/// assert_eq!(polyline5, "cngrIk~inAgtJw~TeoEwtL{sE{{D");
/// assert_eq!(
///     PolylineIter::new(5, &polyline5).collect::<Vec<_>>(),
///     vec![
///         (55.58514, 12.99958),
///         (55.64486, 13.11218),
///         (55.67817, 13.18222),
///         (55.71223, 13.21244)
///     ],
/// );
///
/// // Keeping all the power of working with slices
/// let points = vec![
///     (55.58513, 12.99958),
///     (55.61461, 13.04627),
///     (55.64485, 13.11219),
///     (55.67816, 13.18223),
///     (55.71840, 13.22343),
/// ];
/// assert_eq!(polyline_iter::encode(5, points[1..3].iter().copied()), "ifmrIebsnA_|D_{K");
/// ```
pub struct PolylineIter<'a> {
    polyline: &'a [u8],
    /// Last processed latitude, multiplied by the scale.
    lat: i32,
    /// Last processed longitude, multiplied by the scale.
    lon: i32,
    scale: f64,
}

impl<'a> PolylineIter<'a> {
    /// Creates a new iterator over points decoded from a polyline.
    /// The precision is the number of decimal places in the coordinates, which is 5 for polyline5 and 6 for polyline6.
    pub fn new(precision: u8, polyline: &'a str) -> Self {
        assert!(precision <= 7, "i32 can hold up to 180 * 10^7");
        PolylineIter {
            polyline: polyline.as_bytes(),
            lat: 0,
            lon: 0,
            scale: 10.0_f64.powi(precision as i32),
        }
    }

    #[inline(always)]
    fn varint_decode(&mut self) -> Option<u32> {
        let mut result = 0;
        for i in 0..self.polyline.len() {
            // Casting here to i32 here to provide bad value instead of overflow panicking on bad input.
            let chunk = (self.polyline[i] as i32) - 63;
            result |= (chunk & 0x1f) << (i * 5);
            if chunk & 0x20 == 0 {
                self.polyline = &self.polyline[i + 1..];
                return Some(result as u32);
            }
        }
        None
    }
}

impl Iterator for PolylineIter<'_> {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let lat_change = self.varint_decode()?;
        let lon_change = self.varint_decode()?;
        self.lat += zigzag_decode(lat_change);
        self.lon += zigzag_decode(lon_change);
        let lat = self.lat as f64 / self.scale;
        let lon = self.lon as f64 / self.scale;
        Some((lat, lon))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // There are at least polyline.len() / 12 points as each i32 is encoded in 5 bits per char.
        // And at most polyline.len() / 2 points if each number (2 per point) is encoded only by a single char.
        let len = self.polyline.len();
        (len / 12, Some(len / 2))
    }
}

/// Encodes a sequence of points into a polyline with the given precision.
///
/// ```
/// assert_eq!(polyline_iter::encode(5, [(55.58513, 12.99958), (55.61461, 13.04627)]),"angrIk~inAgwDybH");
/// ```
pub fn encode<It>(precision: u8, points: It) -> String
where
    It: IntoIterator<Item = (f64, f64)>,
{
    assert!(precision <= 7, "i32 can hold up to 180 * 10^7");

    let scale = 10.0_f64.powi(precision as i32);
    let mut result = String::with_capacity(16);

    let mut prev = (0.0, 0.0);
    for point in points {
        let lat_change = ((point.0 - prev.0) * scale).round() as i32;
        let lon_change = ((point.1 - prev.1) * scale).round() as i32;

        varint_encode(zigzag_encode(lat_change), &mut result);
        varint_encode(zigzag_encode(lon_change), &mut result);

        prev = point;
    }
    result
}

/// Zigzag encoded numbers store the sign in the least significant bit, which this function moves to the sign bit.
fn zigzag_decode(i: u32) -> i32 {
    (i >> 1) as i32 ^ -((i & 1) as i32)
}

/// Moves the sign bit from the most significant bit to the least significant bit,
/// thus reducing number of significant bits for negative numbers.
fn zigzag_encode(value: i32) -> u32 {
    (value << 1) as u32 ^ (value >> 31) as u32
}

/// Encodes the value into a variable-length format, storing 5 bits per byte to keep
/// all bytes URL-compatible (from 63 to 126).
fn varint_encode(mut value: u32, buffer: &mut String) {
    while value >= 0x20 {
        let byte = char::from_u32(((value & 0x1F) | 0x20) + 63).unwrap();
        buffer.push(byte);
        value >>= 5;
    }
    let byte = char::from_u32(value + 63).unwrap();
    buffer.push(byte);
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(zigzag_encode(0), 0);
        assert_eq!(zigzag_encode(-1), 1);
        assert_eq!(zigzag_encode(1), 2);
        assert_eq!(zigzag_encode(-2), 3);
        assert_eq!(zigzag_encode(2), 4);
        assert_eq!(zigzag_encode(-3), 5);
        assert_eq!(zigzag_encode(3), 6);
        assert_eq!(zigzag_encode(-4), 7);
        assert_eq!(zigzag_encode(4), 8);
        assert_eq!(zigzag_encode(-5), 9);
        assert_eq!(zigzag_encode(5), 10);
        assert_eq!(zigzag_encode(-6), 11);
        assert_eq!(zigzag_encode(6), 12);
        assert_eq!(zigzag_encode(-7), 13);
        assert_eq!(zigzag_encode(7), 14);
        assert_eq!(zigzag_encode(-8), 15);
    }

    #[test]
    fn empty_polyline() {
        assert_eq!(PolylineIter::new(5, "").next(), None);
        assert_eq!(encode(5, []), "");

        assert_eq!(PolylineIter::new(6, "").next(), None);
        assert_eq!(encode(6, []), "");
    }

    #[test]
    fn single_point() {
        assert_eq!(encode(5, [(0.0, 0.0)]), "??");
        assert_eq!(PolylineIter::new(5, "??").collect::<Vec<_>>(), [(0.0, 0.0)]);
        assert_eq!(encode(6, [(0.0, 0.0)]), "??");
        assert_eq!(PolylineIter::new(6, "??").collect::<Vec<_>>(), [(0.0, 0.0)]);

        let point = (55.71218211778275, 13.21561509233427);
        assert_eq!(encode(5, [point]), "ch`sIsdtoA");
        assert_eq!(
            PolylineIter::new(5, "ch`sIsdtoA").collect::<Vec<_>>(),
            [(55.71218, 13.21562)]
        );
        assert_eq!(encode(6, [point]), "kzkgiB}vreX");
        assert_eq!(
            PolylineIter::new(6, "kzkgiB}vreX").collect::<Vec<_>>(),
            [(55.712182, 13.215615)]
        );
        assert_eq!(encode(7, [point]), "yp_se`@mnda{F");
        assert_eq!(
            PolylineIter::new(7, "yp_se`@mnda{F").collect::<Vec<_>>(),
            [(55.7121821, 13.2156151)]
        );

        let point = (37.82070486887192, -122.47866012130189);
        assert_eq!(encode(5, [point]), "kzyeFrrpjV");
        assert_eq!(
            PolylineIter::new(5, "kzyeFrrpjV").collect::<Vec<_>>(),
            [(37.82070, -122.47866)]
        );
        assert_eq!(encode(6, [point]), "aqkcgAfcorhF");
        assert_eq!(
            PolylineIter::new(6, "aqkcgAfcorhF").collect::<Vec<_>>(),
            [(37.820705, -122.478660)]
        );

        let point = (-54.906532713928094, -65.99208264367125);
        assert_eq!(encode(5, [point]), "x|bnInaxqK");
        assert_eq!(
            PolylineIter::new(5, "x|bnInaxqK").collect::<Vec<_>>(),
            [(-54.90653, -65.99208)]
        );
        assert_eq!(encode(6, [point]), "hifvgBdxyz|B");
        assert_eq!(
            PolylineIter::new(6, "hifvgBdxyz|B").collect::<Vec<_>>(),
            [(-54.906533, -65.992083)]
        );

        let point = (-37.88209074375984, 144.79631245265494);
        assert_eq!(encode(5, [point]), "`zefF}owrZ");
        assert_eq!(
            PolylineIter::new(5, "`zefF}owrZ").collect::<Vec<_>>(),
            [(-37.88209, 144.79631)]
        );
        assert_eq!(encode(6, [point]), "tmcggAohtdsG");
        assert_eq!(
            PolylineIter::new(6, "tmcggAohtdsG").collect::<Vec<_>>(),
            [(-37.882091, 144.796312)]
        );
    }

    #[test]
    fn multiple_points() {
        let polyline = "angrIk~inAgwDybH_|D_{KeoEwtLozFo`Gre@tcA";
        assert!(check_polyline(polyline));
        let mut iter = PolylineIter::new(5, polyline);
        assert_eq!(iter.next(), Some((55.58513, 12.99958)));
        assert_eq!(iter.next(), Some((55.61461, 13.04627)));
        assert_eq!(iter.next(), Some((55.64485, 13.11219)));
        assert_eq!(iter.next(), Some((55.67816, 13.18223)));
        assert_eq!(iter.next(), Some((55.71840, 13.22343)));
        assert_eq!(iter.next(), Some((55.71222, 13.21244)));
        assert_eq!(iter.next(), None);

        // If polyline is decoded with wrong precision, the points will be x10 times smaller or bigger.
        let mut iter = PolylineIter::new(6, polyline);
        assert_eq!(iter.next(), Some((5.558513, 1.299958)));
        assert_eq!(iter.next(), Some((5.561461, 1.304627)));
        assert_eq!(iter.next(), Some((5.564485, 1.311219)));
        assert_eq!(iter.next(), Some((5.567816, 1.318223)));
        assert_eq!(iter.next(), Some((5.571840, 1.322343)));
        assert_eq!(iter.next(), Some((5.571222, 1.321244)));
        assert_eq!(iter.next(), None);

        // Forward and backward transcoding should give the same result
        let polyline = "gzkgiBgwreX{@sI~HcBwBoi@sXvBsIcBgJSwGg@wGg@cG{@{JoAwGSkC{@ce@gOwj@oKsb@cBoFz@gEjC?~RRb[f@v[Sz@kHnAoA_l@SsIR?";
        assert_eq!(encode(6, PolylineIter::new(6, polyline)), polyline);

        // Transcoding polyline to another precision
        assert_eq!(
            encode(5, PolylineIter::new(6, polyline)),
            "ch`sIsdtoAEa@^IKgCqAJa@Ic@A[C[CYEe@G[AMEyBs@kCg@qBIWDSL?~@@xABzAAD]FGoCAa@@?"
        );

        assert_eq!(
            encode(
                6,
                // decoded with wrong precision, but then corrected by `* 10.0`
                PolylineIter::new(7, polyline).map(|(lat, lon)| (lat * 10.0, lon * 10.0))
            ),
            polyline
        );
    }

    #[test]
    #[should_panic]
    fn bad_precision() {
        encode(8, []);
    }

    #[test]
    fn broken_string() {
        // Last point is missing a lon change, so the whole points will be skipped.
        let polyline = "_p~iF~ps|U_ulLnnqC_mqNvxq";
        assert!(!check_polyline(polyline)); // the polyline is not valid, but still can be decoded.
        let mut iter = PolylineIter::new(5, polyline);
        assert_eq!(iter.next(), Some((38.5, -120.2)));
        assert_eq!(iter.next(), Some((40.7, -120.95)));
        assert_eq!(iter.next(), None);

        let mut iter = PolylineIter::new(6, polyline);
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
        let mut iter = PolylineIter::new(5, polyline);
        assert_eq!(iter.next(), None);

        // Now let's add `!` in the middle of a valid polyline.
        let polyline = "_p~iF~ps|U_ulLnnqC!_mqNvxq";
        assert!(!check_polyline(polyline)); // the polyline is not valid, but still can be decoded.
        let mut iter = PolylineIter::new(5, polyline);
        assert_eq!(iter.next(), Some((38.5, -120.2)));
        assert_eq!(iter.next(), Some((40.7, -120.95)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn size_hint() {
        let iter = PolylineIter::new(5, "_p~iF~ps|U_ulLnnqC_mqNvxq`@");
        // Size hint should not be precise as the number of points depends the distance between them.
        assert!(iter.size_hint().0 <= 3);
        assert!(iter.size_hint().1.unwrap() >= 3);
        assert_eq!(iter.count(), 3);
    }
}
