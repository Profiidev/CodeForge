/*use std::{
  sync::{Arc, Condvar, Mutex},
  thread,
};

use async_std::task::spawn;

struct Test {
  a: i32,
  condition: Arc<(Mutex<bool>, Condvar)>,
}

struct Rapper {
  test: Arc<Mutex<Test>>,
}

#[async_std::main]
async fn main() {
  let condition = Arc::new((Mutex::new(false), Condvar::new()));
  let test = Test { a: 0, condition: condition.clone() };
  let test2 = Arc::new(Mutex::new(test));
  let test3 = test2.clone();

  let rapper = Rapper::new(test2);

  spawn(async move {
    let mut i = 0;
    loop {
      i += 1;
      println!("waiting {} times", i);
      let mut test = test3.lock().unwrap();
      if test.a == 1 {
        println!("resolving");
        test.resolve();
        break;
      }
    }
  });

  thread::sleep(std::time::Duration::from_secs(1));

  rapper.req().await;
}

impl Test {
  pub(crate) fn resolve(&mut self) {
    self.a = 2;
    let (lock, cvar) = &*self.condition;
    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_one();
  }
}

impl Rapper {
  pub(crate) fn new(test: Arc<Mutex<Test>>) -> Self {
    Self { test }
  }

  //req
  pub(crate) async fn req(&self) {
    {
      let mut test = self.test.lock().unwrap();
      test.a = 1;
    }

    // Wait for the response
    {
      let (lock, cvar) = &*self.test.lock().unwrap().condition.clone();
      let mut started = lock.lock().unwrap();
      while !*started {
        started = cvar.wait(started).unwrap();
      }
    }
  }
}*/


fn main() {
  let test = "test1";
  //use test as macro input
  let test = test!(test);
}

#[macro_export]
macro_rules! test {
  ("test1") => {
    "test1"
  };
  ("test2") => {
    "test2"
  };
  ($input:expr) => {
    $input
  };
}