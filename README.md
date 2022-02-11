# massa-auto-rebuy
Little executable to run aside of the node to automatically rebuy your roll


## Setup

Have your wallet.dat in the same workspace and then you can run it using :
```
cargo run [YOUR_IP] [YOUR_PUBLIC_PORT]
```

The public port is optional if not provided it will use : 33035.

You can then add it to a crontab or similar to make it run autonomously.

## TODO

- Add logs
- Add loop in the executable