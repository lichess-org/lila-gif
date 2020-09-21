#!/bin/sh -e
cargo +stable build --release --target x86_64-unknown-linux-musl
ssh "root@$1.lichess.ovh" mv /usr/local/bin/lila-gif /usr/local/bin/lila-gif.bak || (echo "first deploy on this server? set up service and comment out this line" && false)
scp ./target/x86_64-unknown-linux-musl/release/lila-gif "root@$1.lichess.ovh":/usr/local/bin/lila-gif
ssh "root@$1.lichess.ovh" systemctl restart lila-gif
