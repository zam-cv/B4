pub struct Bank;
pub struct SentenceBuilder;

impl Bank {
    pub fn new() -> Self {
        Bank
    }

    pub fn create_sentence(&self, sentence_builder: &SentenceBuilder) -> String {
        sentence_builder.build()
    }
}

impl SentenceBuilder {
    pub fn new() -> Self {
        SentenceBuilder
    }

    pub fn build(&self) -> String {
        "This is a sentence".to_string()
    }
}
