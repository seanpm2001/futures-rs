use std::mem;

use {Future, empty, Poll, Task};

impl<T, E> Future for Box<Future<Item=T, Error=E>>
    where T: 'static,
          E: 'static,
{
    type Item = T;
    type Error = E;

    fn poll(&mut self, task: &mut Task) -> Poll<Self::Item, Self::Error> {
        (**self).poll(task)
    }

    unsafe fn tailcall(&mut self)
                       -> Option<Box<Future<Item=Self::Item, Error=Self::Error>>> {
        if let Some(f) = (**self).tailcall() {
            return Some(f)
        }
        Some(mem::replace(self, Box::new(empty())))
    }
}

impl<T, E> Future for Box<Future<Item=T, Error=E> + Send>
    where T: Send + 'static,
          E: Send + 'static,
{
    type Item = T;
    type Error = E;

    fn poll(&mut self, task: &mut Task) -> Poll<Self::Item, Self::Error> {
        (**self).poll(task)
    }

    unsafe fn tailcall(&mut self)
                       -> Option<Box<Future<Item=Self::Item, Error=Self::Error>>> {
        if let Some(f) = (**self).tailcall() {
            return Some(f)
        }
        let me = mem::replace(self, Box::new(empty()));
        let me: Box<Future<Item=T, Error=E>> = me;
        Some(me)
    }
}

impl<F: Future> Future for Box<F> {
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self, task: &mut Task) -> Poll<Self::Item, Self::Error> {
        (**self).poll(task)
    }

    unsafe fn tailcall(&mut self)
                       -> Option<Box<Future<Item=Self::Item, Error=Self::Error>>> {
        (**self).tailcall()
    }
}
