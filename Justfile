watch:
    mold -run cargo watch --workdir /workspace/ -w crates/web-server -w crates/db --no-gitignore -x "run --bin web-server"