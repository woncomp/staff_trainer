@echo off
where wasm-server-runner >nul 2>&1
if %errorlevel% neq 0 (
    echo wasm-server-runner is not installed or not in the PATH.
    echo Please follow the guides bellow to install it
    echo https://bevy-cheatbook.github.io/platforms/wasm.html
    echo or
    echo https://github.com/jakobhellermann/wasm-server-runner
    pause
    exit
)

set CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner
cargo run --target wasm32-unknown-unknown
