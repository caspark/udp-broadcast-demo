UDP Broadcast Demo
==================

A simple program to demo & test UDP broadcasting & receiving - helpful for isolating problems in network stacks provided by game engines.

Running
-------

On the sending and receiving machines, run `ip addr` to make sure they're on the same subnet; in my case, `ip addr` has a bit saying `brd 192.168.0.255` which means `192.168.0.255` is the broadcast address for both machines, so to a first approximation we should be good.

Now let's run the server:

```
cargo run
```

And the client:

```
cargo run -- 192.168.0.255 trying-broadcast
```

If all goes well, you should see output on the client-side of:

```
binding to 0.0.0.0:8901 for sending
broadcasting to 192.168.0.255 data of trying-broadcast
```

and server-side should say:

```
listening on 0.0.0.0:8900...
read 16 bytes from V4(192.168.0.104:8901)
bytes are valid UTF8; string: trying-broadcast
```

Tada! Now you're cooking with UDP.