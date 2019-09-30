extern crate tokio;
extern crate lapin_futures;

use futures::future::Future;
use lapin_futures as lapin;
use crate::lapin_futures::{Client, ConnectionProperties, BasicProperties};
use crate::lapin_futures::options::{QueueDeclareOptions, BasicConsumeOptions, BasicPublishOptions};
use crate::lapin_futures::types::FieldTable;

use std::str;
use tokio::prelude::*;

fn main() {
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    futures::executor::spawn(
        Client::connect(&addr, ConnectionProperties::default()).and_then(|client| {
            // create_channel returns a future that is resolved
            // once the channel is successfully created
            client.create_channel()
        }).and_then(|channel| {
            let id = channel.id();
            println!("created channel with id: {}", id);

            // we using a "move" closure to reuse the channel
            // once the queue is declared. We could also clone
            // the channel
            let dec_queue = channel.queue_declare("hello", QueueDeclareOptions::default(), FieldTable::default());
            let on_queue = dec_queue.then(move |queue| {
                println!("channel {} declared queue {}", id, "hello");

                let queue = queue.unwrap();

                let msg = b"asdfkjasdkdf";

                channel.basic_publish("", "hello", msg.to_vec(), BasicPublishOptions::default(), BasicProperties::default()).wait().expect("publish");
                channel.basic_publish("", "hello", msg.to_vec(), BasicPublishOptions::default(), BasicProperties::default()).wait().expect("publish");
                channel.basic_publish("", "hello", msg.to_vec(), BasicPublishOptions::default(), BasicProperties::default()).wait().expect("publish");

                println!("published");

                let consume = channel.basic_consume(
                    &queue,
                    "",
                    BasicConsumeOptions::default(),
                    FieldTable::default()
                ).then(move |consumer| {
                    /*let consumer = consumer.unwrap();

                    consumer.for_each(move |delivery| {
                        println!("Message: {}", str::from_utf8(&delivery.data).unwrap());
                        channel.basic_ack(delivery.delivery_tag, false);

                        Ok(())
                    })*/
                    Ok(())
                });

                consume
            });

            on_queue
        })
    ).wait_future().expect("runtime failure");
}
