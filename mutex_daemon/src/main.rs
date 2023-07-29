use std::{sync::{Mutex, Arc}, collections::HashMap, borrow::Cow, thread, time::Duration};

use lazy_static::lazy_static;

lazy_static! {
    static ref METRICS: Mutex<HashMap<Cow<'static, str>, usize>> =
        Mutex::new(HashMap::new());
}

fn main() {
    let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));
    for _ in 0.. 232 {
        let m = metrics.clone();
        thread::spawn(move || {
            let mut g = m.lock().unwrap();
            let data = &mut *g;
            let entry = data.entry("hello".into()).or_insert(0);
            *entry += 1;
        });
    }
    thread::sleep(Duration::from_millis(100));
    println!("metrics: {:?}", metrics.lock().unwrap());
}
