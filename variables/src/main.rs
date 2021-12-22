fn main() {
    let mut s = String::from("helloworld");
    let word = first_word(&s);
    println!("{}", word);
    s.clear();

    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
