pub struct Post {}

pub struct Draft {
    content: String,
}

impl Post {
    pub fn new(message: &str) -> Draft {
        Draft {
            content: message.to_owned(),
        }
    }
}

impl Draft {
    pub fn request_review(self) -> PeningReview {
        PeningReview {
            content: self.content,
        }
    }
}

pub struct PeningReview {
    content: String,
}

impl PeningReview {
    pub fn approve(self) -> Published {
        Published {
            content: self.content,
        }
    }
}

pub struct Published {
    content: String,
}

impl Published {
    pub fn content(&self) -> &str {
        &self.content
    }
}
