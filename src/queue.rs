use std::{
    collections::{BTreeMap, VecDeque},
    sync::{
        RwLock,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
};

use reqwest::{RequestBuilder, Response};
use tokio::sync::{Notify, oneshot};

pub(crate) struct Request {
    pub(crate) req: RequestBuilder,
    pub(crate) response_tx: oneshot::Sender<Result<Response, reqwest::Error>>,
}

pub(crate) struct PluralKitQueue {
    queue: RwLock<BTreeMap<u16, VecDeque<Request>>>,
    length: AtomicUsize,
    notify: Notify,
}

impl PluralKitQueue {
    pub(crate) fn new() -> Self {
        Self {
            queue: RwLock::new(BTreeMap::new()),
            length: AtomicUsize::new(0),
            notify: Notify::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.length.load(SeqCst)
    }

    pub(crate) fn push(
        &self,
        req: RequestBuilder,
        priority: u16,
    ) -> oneshot::Receiver<Result<Response, reqwest::Error>> {
        let (response_tx, response_rx) = oneshot::channel();
        {
            let mut queue = self.queue.write().expect("lock poisoned");
            queue
                .entry(priority)
                .or_default()
                .push_back(Request { req, response_tx });
        }
        self.length.fetch_add(1, SeqCst);
        self.notify.notify_one();
        response_rx
    }

    pub(crate) fn retry(&self, req: Request, priority: u16) {
        let mut queue = self.queue.write().expect("lock poisoned");
        queue.entry(priority).or_default().push_front(req);
        self.length.fetch_add(1, SeqCst);
        self.notify.notify_one();
    }

    pub(crate) async fn pop(&self) -> (u16, Request) {
        self.notify.notified().await;

        let mut queues = self.queue.write().expect("lock poisoned");
        let mut entry = queues
            .first_entry()
            .expect("queue shouldn't be empty after notify");

        let prio = *entry.key();
        let item = entry
            .get_mut()
            .pop_front()
            .expect("queue shouldn't be empty after notify");

        // if entry is empty remove it
        if entry.get().is_empty() {
            entry.remove_entry();
        }

        self.length.fetch_sub(1, SeqCst);
        (prio, item)
    }
}

#[cfg(test)]
mod test {
    use reqwest::Client;

    use super::*;

    #[tokio::test]
    async fn it_pops_highest_prio() {
        let queue = PluralKitQueue::new();
        let client = Client::new();
        queue.push(client.get("http://example.com"), 5);
        queue.push(client.get("http://example.tld"), 1);

        let (prio, _) = queue.pop().await;
        assert_eq!(prio, 1);
    }

    #[tokio::test]
    async fn it_puts_retry_item_in_front() -> Result<(), crate::Error> {
        let queue = PluralKitQueue::new();
        let client = Client::new();
        queue.push(client.get("http://example.com"), 1);
        let (dummy_tx, _) = oneshot::channel();
        queue.retry(
            Request {
                req: client.get("http://example.tld/"),
                response_tx: dummy_tx,
            },
            1,
        );

        let (_, req) = queue.pop().await;
        assert_eq!(req.req.build()?.url().to_string(), "http://example.tld/");

        Ok(())
    }

    #[tokio::test]
    async fn it_tracks_len_correctly() {
        let queue = PluralKitQueue::new();
        assert_eq!(queue.len(), 0);

        let client = Client::new();
        queue.push(client.get("http://example.com"), 1);
        assert_eq!(queue.len(), 1);

        let (_, request) = queue.pop().await;
        assert_eq!(queue.len(), 0);

        queue.retry(request, 1);
        assert_eq!(queue.len(), 1);
    }
}
