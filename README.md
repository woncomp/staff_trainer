# staff_trainer


# Development

Bevy dynamic_linking is only enabled with the binary "fastdev", run these commands on windows when you want to iterate faster during development:

> dev.bat run
> dev.bat check

# Building WASM

> cargo install wasm-bindgen-cli
> build_wasm.bat

# Deploying WASM to Github Pages

> git checkout web
> git add assets
> git add dist
> git commit
> git push origin web