use bson;
use mongodb;

use iron::IronError;
use iron::status::Status;

use hyper;
use hyper::error::UriError;
use url;

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
        Io(::std::io::Error);
        Hyper(hyper::Error);
        HyperUri(UriError);
        Url(url::ParseError);
    }

    errors {
        RouterArgumentIsNotProvided(argument_name: String) {
            description("Router argument is not provided")
            display("Router argument is not provided: '{}'", argument_name)
        }
    }
}

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        IronError::new(err, Status::InternalServerError)
    }
}
