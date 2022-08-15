use huff::encode;

fn main() {
    let table = encode("stranger in a strange land");

    for (character, encoded) in table {
        println!("{} {}", character, encoded);
    }
}
