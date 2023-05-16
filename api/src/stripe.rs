use hub_core::clap;
pub use stripe::Client;

/// Arguments for instantiating a Stripe client
#[derive(Debug, clap::Args)]
pub struct StripeArgs {
    #[arg(long, env)]
    pub stripe_secret_key: String,
    #[arg(long, env)]
    pub stripe_webhook_secret: String,
}

#[derive(Clone)]
pub struct Stripe {
    pub client: Client,
    pub webhook_secret: String,
}

impl Stripe {
    #[must_use]
    pub fn new(secret_key: String, webhook_secret: String) -> Self {
        let client = Client::new(secret_key);

        Self {
            client,
            webhook_secret,
        }
    }
}
