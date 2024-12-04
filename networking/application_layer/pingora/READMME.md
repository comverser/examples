# Pingora

<https://youtu.be/avg2w1-fyae>

- Reverse proxy
- Load balancer
- Modify request
- Modify response

## Reverse proxy example

```bash
PORT=3000 cargo run --bin server --features="server"
```

```bash
RUST_LOG=INFO cargo run --bin reverse_proxy --features="proxy"
```

## Load balancer example

```bash
PORT=3000 cargo run --bin server --features="server"
```

```bash
PORT=4000 cargo run --bin server --features="server"
```

```bash
RUST_LOG=INFO cargo run --bin load_balancer --features="proxy"
```

## Gateway

```bash
PORT=3000 cargo run --bin server --features="server"
```

```bash
PORT=4000 cargo run --bin server --features="server"
```

```bash
RUST_LOG=INFO cargo run --bin gateway --features="proxy"
```
