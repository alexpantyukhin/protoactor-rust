extern crate futures;
use futures::Future;
use std::{io, str};
use std::time::Duration;
use std::collections::HashMap;

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

pub struct LocalContext<'a> {
    _producer : fn() -> &'a Actor,
    _receiveMiddleware: fn(context: Context) -> Future<Item=(), Error = io::Error>,
    _parent: PID
}

pub trait Context {
    fn parent(&self) -> &PID;
    fn slf(&self) -> &PID;
    fn sender(&self) -> &PID;
    fn actor(&self) -> &Actor;
}


impl<'a> LocalContext<'a> {
    fn new(producer: fn() -> &'a Actor, parent: PID, receiveMiddleware: fn(context: Context) -> Future<Item=(), Error = io::Error>) -> LocalContext {
        LocalContext {
            _producer: producer,
            _parent: parent,
            _receiveMiddleware: receiveMiddleware
        }
    }
}

impl<'a> Context for LocalContext<'a> {
    fn parent(&self) -> &PID {
        &self._parent
    }

    fn slf(&self) -> &PID {
        //TODO :return self pid
        &self._parent
    }

    fn sender(&self) -> &PID {
        // TODO: return sender pid
        &self._parent
    }

    fn actor(&self) -> &Actor {
        (self._producer)()
    }
}