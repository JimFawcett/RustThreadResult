# RustThreadResult
##Support for returning results from threads

ThreadResult<T> is similar to the C++ Future type.  It has no hooks into a run-time like Tokio.  Instead it simply causes main thread access to a result to block until the child thread signals that the result is ready.  Threads needing the result may poll to see if the result is ready rather than blocking. 
```
/////////////////////////////////////////////////////////////
// thread_result::lib.rs - Wait for thread to complete     //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 14 Apr 2021  //
/////////////////////////////////////////////////////////////

#![allow(clippy::mutex_atomic)]
#![allow(dead_code)]
use std::sync::*;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct ThreadResult<T> {
    pub result: Mutex<T>,
    cv: Condvar,
    ready: Mutex<bool>
}

impl<T: Debug + Default + Clone> ThreadResult<T> {
    pub fn new() -> Self {
        Self {
            result: Mutex::new(T::default()),
            cv: Condvar::new(),
            ready: Mutex::new(false),
        }
    }
    /*--------------------------------------------
      Unwrapping is appropriate here.  The 
      operation fails if the Mutex becomes
      poisoned, due to panic on a thread
      holding the lock.  But then you can't
      do much except quit, which the unwrap
      does for you.
    --------------------------------------------*/
    pub fn set(&self, t:T) {
        let mut lr = self.ready.lock().unwrap();
        *lr = true;
        let mut lrslt = self.result.lock().unwrap();
        *lrslt = t;
        self.cv.notify_one();
    }
    pub fn get(&self) -> T {
        let mut rdy = self.ready.lock().unwrap();
        while !*rdy {
            rdy = self.cv.wait(rdy).unwrap();
        }
        let rslt = self.result.lock().unwrap();
        rslt.clone()
    }
    pub fn ready(&self) -> bool {
        *self.ready.lock().unwrap()
    }
}
```
