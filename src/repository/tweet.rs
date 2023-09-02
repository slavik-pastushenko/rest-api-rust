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
        let date = Some(Utc::now());

        let tweet = Tweet { id: Some(id), created_at: date, updated_at: date, ..tweet };

        tweets.push(tweet.clone());

        Ok(tweet)
    }

    pub fn update_tweet(&self, id: &str, tweet: Tweet) -> Option<Tweet> {
        let mut tweets = self.tweets.lock().unwrap();

        let mut tweet = Tweet { id: Some(id.to_string()), updated_at: Some(Utc::now()), ..tweet };
        let index = tweets.iter().position(|tweet| tweet.id == Some(id.to_string()))?;

        tweet.created_at = tweets[index].created_at;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_database() {
        let db = Database::new();
        let tweets = db.tweets.lock().unwrap();

        assert_eq!(tweets.len(), 0);
    }

    #[test]
    fn test_create_tweet() {
        let db = Database::new();
        let tweet = Tweet {
            id: None,
            message: "Hello, World!".to_string(),
            updated_at: None,
            created_at: None,
        };

        let created_tweet = db.create_tweet(tweet.clone()).unwrap();

        assert_eq!(created_tweet.message, tweet.message);
        assert!(created_tweet.created_at.is_some());
    }

    #[test]
    fn test_update_tweet() {
        let db = Database::new();
        let tweet = Tweet {
            id: None,
            message: "Message".to_string(),
            updated_at: None,
            created_at: None,
        };

        let created_tweet = db.create_tweet(tweet.clone()).unwrap();
        let updated_tweet = db
            .update_tweet(created_tweet.id.as_ref().unwrap(), Tweet {
                id: None,
                message: "Message 2".to_string(),
                updated_at: None,
                created_at: None,
            })
            .unwrap();

        assert_eq!(updated_tweet.message, "Message 2");
        assert!(updated_tweet.updated_at.is_some());
    }

    #[test]
    fn test_delete_tweet() {
        let db = Database::new();
        let tweet = Tweet {
            id: None,
            message: "Message".to_string(),
            updated_at: None,
            created_at: None,
        };

        let created_tweet = db.create_tweet(tweet.clone()).unwrap();
        let deleted_tweet = db.delete_tweet(created_tweet.id.as_ref().unwrap()).unwrap();

        assert_eq!(deleted_tweet.id, created_tweet.id);
        assert_eq!(deleted_tweet.message, created_tweet.message);
        assert!(deleted_tweet.updated_at.is_some());
    }

    #[test]
    fn test_get_tweets() {
        let db = Database::new();
        let tweet = Tweet {
            id: None,
            message: "Message".to_string(),
            updated_at: None,
            created_at: None,
        };

        let created_tweet = db.create_tweet(tweet.clone()).unwrap();
        let tweets = db.get_tweets();

        assert_eq!(tweets[0].id, created_tweet.id);
        assert_eq!(tweets[0].message, created_tweet.message);
        assert!(tweets[0].updated_at.is_some());
        assert!(tweets[0].created_at.is_some());
    }

    #[test]
    fn test_get_tweet_by_id() {
        let db = Database::new();
        let tweet = Tweet {
            id: None,
            message: "Message".to_string(),
            updated_at: None,
            created_at: None,
        };

        let created_tweet = db.create_tweet(tweet.clone()).unwrap();
        let tweet = db.get_tweet_by_id(&created_tweet.id.as_ref().unwrap()).unwrap();

        assert_eq!(tweet.id, created_tweet.id);
        assert_eq!(tweet.message, created_tweet.message);
        assert!(tweet.updated_at.is_some());
        assert!(tweet.created_at.is_some());
    }
}
