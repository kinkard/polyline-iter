# About

Zero-dependency Rust crate for encoding and decoding [Google's polyline format](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).

Compared to the [georust/polyline](https://github.com/georust/polyline) crate, the `polyline-iter` decodes polyline into an iterator over points instead of vector, which is benefitial when only a single iteration over the polyline is needed. And in such cases it performs twice as fast as the `georust/polyline` crate and has no hidden allocations.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
polyline-iter = "0.3"
```

## Example

```rust
use polyline_iter::PolylineIter;

let iter = PolylineIter::new(6, "avs_iB}xlxWissBw|zEu``AsxgCyoaAm_z@");
assert_eq!(
    iter.collect::<Vec<_>>(),
    vec![
        (55.585137, 12.999583),
        (55.644854, 13.112187),
        (55.678161, 13.182229),
        (55.712222, 13.212444),
    ]
);

// If the points are not needed, the iterator can be used directly
assert_eq!(PolylineIter::new(5, "avs_iB}xlxWissBw|zEu``AsxgCyoaAm_z@").count(), 4);

// Iterator approach allows to transcode polyline to another precision without intermediate allocations.
let polyline5 = polyline_iter::encode(5, PolylineIter::new(6, "avs_iB}xlxWissBw|zEu``AsxgCyoaAm_z@"));
assert_eq!(polyline5, "cngrIk~inAgtJw~TeoEwtL{sE{{D");
assert_eq!(
    PolylineIter::new(5, &polyline5).collect::<Vec<_>>(),
    vec![
        (55.58514, 12.99958),
        (55.64486, 13.11218),
        (55.67817, 13.18222),
        (55.71223, 13.21244)
    ],
);

// Keeping all the power of working with slices
let points = vec![
    (55.58513, 12.99958),
    (55.61461, 13.04627),
    (55.64485, 13.11219),
    (55.67816, 13.18223),
    (55.71840, 13.22343),
];
assert_eq!(polyline_iter::encode(5, points[1..3].iter().copied()), "ifmrIebsnA_|D_{K");
```

## License

All code in this project is dual-licensed under either:

- [MIT license](https://opensource.org/licenses/MIT) ([`LICENSE-MIT`](LICENSE-MIT))
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([`LICENSE-APACHE`](LICENSE-APACHE))

at your option.
