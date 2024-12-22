# About

Zero-dependency Rust crate for decoding [Google's polyline format](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).

Compared to the [georust/polyline](https://github.com/georust/polyline) crate, this one does not store the decoded points in a `Vec` but instead provides an iterator over the points, allowing user to decide how to store the points or not store them at all if they are not needed.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
polyline-iter = "0.1"
```

## Example

```rust
use polyline_iter::PolylineIter;

let iter = PolylineIter::new(5, "angrIk~inAgwDybH_|D_{KeoEwtLozFo`Gre@tcA");
assert_eq!(
    iter.collect::<Vec<_>>(),
    vec![
        (55.58513, 12.99958),
        (55.61461, 13.04627),
        (55.64485, 13.11219),
        (55.67816, 13.18223),
        (55.71840, 13.22343),
        (55.71222, 13.21244),
    ]
);

// If the points are not needed, the iterator can be used directly
assert_eq!(PolylineIter::new(5, "angrIk~inAgwDybH_|D_{KeoEwtLozFo`Gre@tcA").count(), 6);
```

## License

All code in this project is dual-licensed under either:

- [MIT license](https://opensource.org/licenses/MIT) ([`LICENSE-MIT`](LICENSE-MIT))
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([`LICENSE-APACHE`](LICENSE-APACHE))

at your option.
