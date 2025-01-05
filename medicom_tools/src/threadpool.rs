/*
   Copyright 2024-2025 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! This is a simple thread pool implementation, largely based on the rust book.
//!
//! <https://doc.rust-lang.org/book/ch20-02-multithreaded.html>
//! <https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html>

// The thread pool is currently only used by the SCP Provider app, which is optional. If compiled
// without that feature this thread pool module becomes unused, but there's no reason to throw up
// warnings about it, or to only conditionally include this module in case it ends up being useful
// elsewhere in the future.
#![allow(dead_code)]

use anyhow::{anyhow, Result};

use std::{
    sync::{
        mpsc::{channel, Receiver, SendError, Sender},
        Arc, Mutex,
    },
    thread::{spawn, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + Sync + 'static>;

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = spawn(move || loop {
            match Worker::await_job(&receiver) {
                Ok(job) => {
                    job();
                }
                Err(e) => {
                    eprintln!("Worker {id} disconnected, shutting down. {e:?}");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }

    /// Awaits for a job, ensuring lock is released after job is received.
    fn await_job(receiver: &Arc<Mutex<Receiver<Job>>>) -> Result<Job> {
        let job = receiver
            .lock()
            .map_err(|_e| anyhow!("Failed to acquire lock to receive job."))?
            .recv()
            .map_err(|_e| anyhow!("Failed to receive job."))?;
        Ok(job)
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), SendError<Job>>
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = &self.sender {
            sender.send(job)
        } else {
            Err(SendError(job))
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                match thread.join() {
                    Ok(()) => {}
                    Err(e) => eprintln!("Failed to shut down worker {}: {e:?}", worker.id),
                }
            }
        }
    }
}
