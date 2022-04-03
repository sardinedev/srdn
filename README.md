# srdn
The Sardine CLI

## Run locally

```bash
cargo run -- --option [command]
```

### Builds all files in `tests/src` and outputs them in `tests/dist`
```bash
cargo run -- build -s tests/src -d tests/dist
```

### Builds one file and outputs it in `tests/dist/a/a.css`
```bash
cargo run -- build -f ./tests/src/a/a.module.css -o tests/dist/a/a.css
```