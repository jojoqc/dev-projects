use database::Database;
use iron::headers::ContenType;
use iron::{status, AfterMiddleware, Hander, Ironresult, Request, Response};
use models::Post;
use router::Router;
use rustc_serialize::json;
use std::error::Error;
use std::io::Read;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

macro_rules! try_handler {
    ($e: expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                return Ok(Response::with((
                    status::InternalServerError,
                    e.descripcion(),
                )))
            }
        }
    };
    ($e:expr, $error:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with(($error, e.description()))),
        }
    };
}
macro_rules! lock {
    ($e:expr) => {
        e.lock().unwrap()
    };
}
macro_rules! get_http_param {
    ($r:expr,$e:expr) => {
        match $r.extensions.get::<Router()>{
            Some(router)=>{
                Some(v)=>v,
                None()=>return Ok(Response::with(status::BadRequest)),
            }
        },
        None => return Ok(Response::with(status::InternalServerError))
    };
}

pub struct Handlers {}
