type Document = Vec<String>;

// new_document() is a constructor function and it takes ownership of the vector.
// This means Document is the owner of the vector.
fn new_document(words: Vec<String>) -> Document {
    words
}

// add_word requires a mutable reference &mut Document to be able to mutate a
// document. It also consumes ownership of the input word, meaning no one else 
// can mutate the individual words of the document.
fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

// get_words returns an explicit immutable reference to strings within the document. 
// The only way to create a new document from this word vector is to deep-copy 
// its contents as shown in main().
fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}

fn main() {
    let words = vec!["Hello".to_string()];
    let d = new_document(words);

    // .to_vec() converts &[String] to Vec<String> by cloning each element.
    let words_deep_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_deep_copy);

    add_word(&mut d2, "World".to_string());

    // The modification to d2 is not reflected in d.
    assert!(!get_words(&d).contains(&"World".into()));
}
