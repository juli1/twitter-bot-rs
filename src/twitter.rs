use oauth::Token;
use twitter_api;

pub struct Twitter<'a> {
    consumer_token : Token<'a>,
    access_token : Token<'a>,
}

impl<'a> Twitter<'a>{
    pub fn new (consumer_key : String, consumer_secret : String,
                access_key : String, access_secret : String) -> Self {
        Twitter {
            consumer_token : Token::new(consumer_key, consumer_secret),
            access_token : Token::new(access_key, access_secret)
        }
    }

    pub fn tweet (&self, message : String)  {
        twitter_api::update_status(&self.consumer_token, &self.access_token, &message).unwrap();
    }
}

