set shell := ["nu", "-c"]

watch:
    watchexec --exts=rs --on-busy-update=restart -- cargo run
