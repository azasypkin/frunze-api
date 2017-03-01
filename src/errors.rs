use bson;
use mongodb;

use iron::IronError;
use iron::status::Status;

use serde_json;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        BsonEncoder(bson::EncoderError);
        BsonDecoder(bson::DecoderError);
        Mongo(mongodb::Error);
        Serde(serde_json::Error);
    }
}

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        IronError::new(err, Status::InternalServerError)
    }
}
