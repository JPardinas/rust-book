fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    println!("word is {} and s is {}", word, s);

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    first_word_slice_example();
    first_word_slice_improved_example();
}

fn first_word_slice_example() {
    /*
     Error when compiling


    let mut s = String::from("hello world");

    let word = first_word_slice(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
    */

    let s = String::from("hello world");

    let word = first_word_slice(&s);

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice_improved_example() {
    let my_string = String::from("hello world");

    // `first_word_slice_improved` works on slices of `String`s, whether partial or whole
    let word = first_word_slice_improved(&my_string[0..6]);
    println!("the word is: {}", word);
    let word = first_word_slice_improved(&my_string[..]);
    println!("the word is: {}", word);
    // `first_word_slice_improved` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_slice_improved(&my_string);
    println!("the word is: {}", word);

    let my_string_literal = "hello world";

    // `first_word_slice_improved` works on slices of string literals, whether partial or whole
    let word = first_word_slice_improved(&my_string_literal[0..6]);
    println!("the word is: {}", word);
    let word = first_word_slice_improved(&my_string_literal[..]);
    println!("the word is: {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice_improved(my_string_literal);

    println!("the word is: {}", word);
}

fn first_word_slice_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
