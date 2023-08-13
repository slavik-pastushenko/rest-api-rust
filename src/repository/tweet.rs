use std::fmt::Error;
use chrono::prelude::*;
use std::sync::{ Arc, Mutex };

use crate::models::tweet::Tweet;

pub struct Database {
    pub tweets: Arc<Mutex<Vec<Tweet>>>,
}

impl Database {
    pub fn new() -> Self {
        let tweets = Arc::new(Mutex::new(vec![]));

        Database { tweets }
    }

    pub fn create_tweet(&self, tweet: Tweet) -> Result<Tweet, Error> {
        let mut tweets = self.tweets.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let date = Utc::now();

        let tweet = Tweet { id: Some(id), created_at: date, updated_at: date, ..tweet };

        tweets.push(tweet.clone());

        Ok(tweet)
    }

    pub fn update_tweet(&self, id: &str, tweet: Tweet) -> Option<Tweet> {
        let mut tweets = self.tweets.lock().unwrap();

        let updated_at = Utc::now();

        let tweet = Tweet { id: Some(id.to_string()), updated_at, ..tweet };
        let index = tweets.iter().position(|tweet| tweet.id == Some(id.to_string()))?;

        tweets[index] = tweet.clone();

        Some(tweet)
    }

    pub fn delete_tweet(&self, id: &str) -> Option<Tweet> {
        let mut tweets = self.tweets.lock().unwrap();

        let index = tweets.iter().position(|tweet| tweet.id == Some(id.to_string()))?;

        Some(tweets.remove(index))
    }

    pub fn get_tweets(&self) -> Vec<Tweet> {
        let tweets = self.tweets.lock().unwrap();

        tweets.clone()
    }

    pub fn get_tweet_by_id(&self, id: &str) -> Option<Tweet> {
        let tweets = self.tweets.lock().unwrap();

        tweets
            .iter()
            .find(|tweet| tweet.id == Some(id.to_string()))
            .cloned()
    }
}
