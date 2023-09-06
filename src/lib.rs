pub mod message_bus;

use std::collections::HashMap;

/// Events and subscribers
///
/// A library for fun to allow objects to subscribe to events published from other sources

#[allow(dead_code)]
#[allow(unused)]

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Event {
    SomeEvent,
    AnotherEvent
}

type Subscriber = fn(Event);

struct Publisher {
    bus: message_bus::MessageBus,
    subscriptions: HashMap<Event, Vec<Subscriber>>
}

impl Publisher {
    fn new() -> Self {
        Publisher{
            bus: message_bus::MessageBus::new(),
            subscriptions: HashMap::new(),
        }
    }

    fn notify(&mut self, event: Event) {
        let subscribers = self.subscriptions.entry(event.clone()).or_default();

        for sub in subscribers {
            let s = sub.clone();
            let e = event.clone();
            self.bus.queue(move || { s(e) })
        }
    }

    fn subscribe(&mut self, event_type: Event, subscriber: Subscriber) {
        let v = self.subscriptions.entry(event_type.clone()).or_default();
        v.push(subscriber);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn some_subscriber(event: Event) {
        println!("I am some subscriber handling event {:?}!", event)
    }

    fn another_subscriber(event: Event) {
        println!("I am another subscriber handling event {:?}!", event)
    }

    fn third_subscriber(event: Event) {
        println!("I am the third subscriber, handling event: {:?}", event)
    }

    #[test]
    fn notifies_subscribers() {
        let subscriptions: HashMap<Event, Vec<Subscriber>> = HashMap::new();
        let mut publisher = Publisher::new();

        publisher.subscribe(Event::SomeEvent, some_subscriber);
        publisher.subscribe(Event::SomeEvent, another_subscriber);
        publisher.subscribe(Event::SomeEvent, third_subscriber);

        publisher.notify(Event::SomeEvent);
    }
}
