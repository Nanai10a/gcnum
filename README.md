# gcnum

> **g**eneric **c**onstant **num**ber

## usage

```sh
cargo add --git https://github.com/Nanai10a/gcnum.git
```

```rs
use gcnum::Usize;
use serde::Deserialize;

#[derive(Deserialize)]
enum Schema<'a> {
    Message {
        ty: Usize<2>,
        msg: &'a str,
        user_id: u64,
        timestamp: &'a str,
    },
    Ping {
        ty: Usize<1>,
    },
}

fn main() {
    let ping = serde_json::to_string(&Schema::Ping { ty: Usize::<1> }).unwrap();
    println!("{}", ping);
    // {"type":1}

    let message = serde_json::to_string(&Schema::Message {
        ty: Usize::<2>,
        msg: "ping!",
        user_id: 1234567890,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
    .unwrap();
    println!("{}", message);
    // {"type":2,"msg":"ping!","user_id":1234567890,"timestamp":"1996-12-19T16:39:57-08:00"}
}
```
