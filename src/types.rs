use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct IndexElement {
    pub bookId: String,
    pub offset: u64,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Deserialize)]
pub struct Book {
    pub bookId: String,
    title: String,
    series: String,
    author: String,
    rating: String,
    description: String,
    language: String,
    isbn: String,
    genres: String,
    characters: String,
    bookFormat: String,
    edition: String,
    pages: String,
    publisher: String,
    publishDate: String,
    firstPublishDate: String,
    awards: String,
    numRatings: String,
    ratingsByStars: String,
    likedPercent: String,
    setting: String,
    coverImg: String,
    bbeScore: String,
    bbeVotes: String,
    price: String,
}
