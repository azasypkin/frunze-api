use iron::{ AfterMiddleware, headers};
use iron::method::Method;
use iron::method::Method::*;
use iron::prelude::*;
use iron::status::Status;

use unicase::UniCase;

pub struct CORSMiddleware;

impl CORSMiddleware {
    pub fn add_headers(res: &mut Response) {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        res.headers.set(headers::AccessControlAllowHeaders(
            vec![
                UniCase(String::from("accept")),
                UniCase(String::from("authorization")),
                UniCase(String::from("content-type"))
            ]
        ));
        res.headers.set(headers::AccessControlAllowMethods(
            vec![Get, Post, Put, Delete]
        ));
    }
}


impl AfterMiddleware for CORSMiddleware {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        if req.method == Method::Options {
            res = Response::with(Status::Ok);
        }

        CORSMiddleware::add_headers(&mut res);

        Ok(res)
    }

    fn catch(&self, _: &mut Request, mut err: IronError) -> IronResult<Response> {
        CORSMiddleware::add_headers(&mut err.response);

        Err(err)
    }
}