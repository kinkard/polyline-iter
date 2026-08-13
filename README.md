# About

Zero-dependency Rust crate for encoding and decoding [Google's polyline format](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).

Compared to the [georust/polyline](https://github.com/georust/polyline) crate, the `polyline-iter` decodes polyline into an iterator over points instead of vector, which is benefitial when only a single iteration over the polyline is needed. And in such cases it performs twice as fast as the `georust/polyline` crate (check `cargo bench` for exact numbers) and has no hidden allocations.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
polyline-iter = "0.3"
```

## Example

```rust
let iter = polyline_iter::decode(6, "avs_iB}xlxWissBw|zEu``AsxgCyoaAm_z@");
assert_eq!(
    iter.collect::<Vec<_>>(),
    vec![
        (55.585137, 12.999583),
        (55.644854, 13.112187),
        (55.678161, 13.182229),
        (55.712222, 13.212444),
    ]
);

// Count points without collecting them
assert_eq!(polyline_iter::decode(5, "avs_iB}xlxWissBw|zEu``AsxgCyoaAm_z@").count(), 4);

// Iterator approach allows to transcode polyline to another precision without intermediate allocations.
let polyline5 = polyline_iter::encode(5, polyline_iter::decode(6, "avs_iB}xlxWissBw|zEu``AsxgCyoaAm_z@"));
assert_eq!(polyline5, "cngrIk~inAetJy~TeoEwtL{sEy{D");
assert_eq!(
    polyline_iter::decode(5, &polyline5).collect::<Vec<_>>(),
    vec![
        (55.58514, 12.99958),
        (55.64485, 13.11219),
        (55.67816, 13.18223),
        (55.71222, 13.21244)
    ],
);

// Keeping all the power of working with slices
let points = vec![
    (55.58514, 12.99958),
    (55.64485, 13.11219),
    (55.67816, 13.18223),
    (55.71222, 13.21244)
];
assert_eq!(polyline_iter::encode(5, points[1..3].iter().copied()), "icsrIe~_oAeoEwtL");

// Alternatively to URL-compatible polyline format, binary format can be used for more compact representation.
let polyline: String = polyline_iter::encode(5, points.iter().copied());
let binary: Vec<u8> = polyline_iter::encode_binary(5, points);
assert!(binary.len() < polyline.len()); // 20-30% smaller

let transcoded = polyline_iter::encode(5, polyline_iter::decode_binary(5, &binary));
assert_eq!(transcoded, polyline);
```

## License

Dual-licensed under [MIT](https://github.com/kinkard/polyline-iter/blob/main/LICENSE-MIT) or [Apache-2.0](https://github.com/kinkard/polyline-iter/blob/main/LICENSE-APACHE) at your option.
