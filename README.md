# png-util
A zero-copy PNG reader and lazy-copy PNG builder for working with the PNG format.

> [!WARNING]
> This project is in early development. Things may change. Sorry for any inconvenience.

# Usage
This crate offers a PNGReader, which can be iterated over to give you the chunks in order, or collected into a `Vec<ChunkRefs>`
which is a 
```rust
fn read_png() {
    let png_file = std::fs::read("ferris.png").expect("Could not read png file");
    let png = PNGReader::new(&png_file[..]).expect("Could not validate PNG.");

    for chunk in png {
        println!("{:?}", chunk)
    }
}
```