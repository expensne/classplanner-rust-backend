set dotenv-load

run:
    #!/bin/bash
    set -euxo pipefail
    
    cargo run --release

build-local:
    #!/bin/bash
    set -euxo pipefail

    cargo build --release

build-linux:
    #!/bin/bash
    set -euxo pipefail

    rustup target add x86_64-unknown-linux-gnu
    cargo build --release --target x86_64-unknown-linux-gnu

publish: build-linux
    #!/bin/bash
    set -euxo pipefail

    rsync -r -e "ssh -i $SSH_ID" target/x86_64-unknown-linux-gnu/release/classplanner-rust-backend $SSH_DST:~/api-rust/
    rsync -r -e "ssh -i $SSH_ID" .env $SSH_DST:~/api-rust/