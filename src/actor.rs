use actix::prelude::*;

pub struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;
}

pub struct Ping;

impl Message for Ping {
    type Result = String;
}

impl Handler<Ping> for MyActor {
    type Result = String;

    fn handle(&mut self, _: Ping, _: &mut Context<Self>) -> Self::Result {
        "Pong".to_string()
    }
}