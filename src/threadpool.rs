use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// Apply a function to elements of an iterator in parallel
///
/// Consumes the entire iterator immediately.
/// The resulting iterator yields elements in an arbitrary order.
///
/// # Arguments
/// * `iter` - Iterator over elements to map. All order is lost.
/// * `f` - The function to apply at each element.
/// * `num_workers` - Uses a thread pool with this many worker threads.
///
pub fn map<I, F, Y>(iter: I, f: F, num_workers: usize) -> RecvIterator<(<I as Iterator>::Item, Y)>
where
    I: Iterator,
    <I as Iterator>::Item: Send + Clone + 'static,
    F: Fn(<I as Iterator>::Item) -> Y + Clone + Send + 'static,
    Y: Send + 'static,
{
    let (x_sender, x_receiver) = mpsc::channel::<Option<<I as Iterator>::Item>>();
    let x_receiver = Arc::new(Mutex::new(x_receiver));
    let (y_sender, y_receiver) = mpsc::channel();

    for _ in 0..num_workers {
        let f = f.clone();
        let rx = Arc::clone(&x_receiver);
        let tx = y_sender.clone();
        let worker = move || loop {
            let x = match rx.lock().unwrap().recv().unwrap() {
                Some(x) => x,
                None => break,
            };
            tx.send((x.clone(), f(x))).unwrap();
        };
        thread::spawn(worker);
    }

    for x in iter {
        x_sender.send(Some(x)).unwrap();
    }
    // Send None at the end to shut down the workers
    for _ in 0..num_workers {
        x_sender.send(None).unwrap();
    }

    RecvIterator {
        receiver: y_receiver,
    }
}

/// Iterate over values received from a channel.
pub struct RecvIterator<T> {
    receiver: mpsc::Receiver<T>,
}

impl<T> Iterator for RecvIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv().ok()
    }
}
