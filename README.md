# Holaplex Hub Credits
Credits is a chain agnostic method of paying for blockchain transactions and storage using the Holaplex Hub API without having to have a wallet or purchase the blockchain's native token.

# Getting Started

Requires:
- Docker
- Rust

```
$ docker compose up -d
$ cargo run --bin hub-credits
```

Set stripe secrets for in `.env.local` to enable fulfillment of credit purchases using stripe.

```
STRIPE_SECRET_KEY=
STRIPE_WEBHOOK_SECRET=
```
