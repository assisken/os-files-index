use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Book {
    pub bookId: String,
    pub title: String,
    pub series: String,
    pub author: String,
    pub rating: String,
    pub description: String,
    pub language: String,
    pub isbn: String,
    pub genres: String,
    pub characters: String,
    pub bookFormat: String,
    pub edition: String,
    pub pages: String,
    pub publisher: String,
    pub publishDate: String,
    pub firstPublishDate: String,
    pub awards: String,
    pub numRatings: String,
    pub ratingsByStars: String,
    pub likedPercent: String,
    pub setting: String,
    pub coverImg: String,
    pub bbeScore: String,
    pub bbeVotes: String,
    pub price: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexRecord {
    pub bookId: String,
    pub offset: u64,
}

pub type Index = Vec<IndexRecord>;
