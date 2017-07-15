extern crate futures;
use futures::Future;
use std::{io, str};
use std::time::Duration;

pub trait Actor {
    fn receive(&self, context: Context) -> Future<Item = (), Error = io::Error>;
}

pub struct PID {
    address: String,
    id: String
}


impl PID {
    pub fn new (address: String, id: String) -> PID {
        PID {
            address :address,
            id: id
        }
    }
}

pub struct Context<'a> {
    parent : PID,
    slf : PID,
    sender : PID,
    actor: &'a Actor,
    receiveTimeout : Duration,
    children: Vec<PID>
}