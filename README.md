# multi-spinner

A simple terminal spinner library that uses a shared stdout. 

# setup

Smack this into your toml


```toml
[dependencies]
multi-spinner = "0.1.0"
```

Throw this on your code 


```rust
use std::{thread, time::Duration};
use multi_spinner::{Spinner, spinners::Animation};

fn main() {
    let mut spinner = Spinner::builder()
        .msg("awesome".into())
        .start();

    thread::sleep(Duration::from_secs(3));

    spinner.stop().expect("");
}
```

If you need full control over stdout, you can create a mutexed arc


```rust
    let stdout = Arc::new(Mutex::new(stdout()));

    let mut spinner = Spinner::builder()
        .stdout(stdout)
        .msg("splendid".into())
        .start();
```

You can if you want to, change the awesome spinner

```rust
    let mut spinner = Spinner::builder()
        .spinner(Animation::Bars10(0))
        .build();

    spinner.start() // <-- in case you need to await the start
```

