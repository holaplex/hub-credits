//!

use async_std::stream::StreamExt;
use holaplex_hub_credits::{
    build_schema, credits,
    db::Connection,
    events,
    handlers::{get_organization, graphql_handler, health, playground},
    Actions, AppState, Args, Services,
};
use hub_core::{
    anyhow::Context as AnyhowContext,
    tokio::{self, task},
    tracing::{info, warn},
};
use poem::{get, listener::TcpListener, middleware::AddData, post, EndpointExt, Route, Server};

pub fn main() {
    let opts = hub_core::StartConfig {
        service_name: "hub-credits",
    };

    hub_core::run(opts, |common, args| {
        let Args {
            port,
            db,
            gift_amount,
        } = args;

        common.rt.block_on(async move {
            let connection = Connection::new(db)
                .await
                .context("failed to get database connection")?;

            let schema = build_schema();
            let producer = common.producer_cfg.build::<credits::CreditsEvent>().await?;

            let credits = common.credits_cfg.build::<Actions>().await?;
            let state = AppState::new(schema, connection.clone(), credits.clone());

            let cons = common.consumer_cfg.build::<Services>().await?;
            let conn = connection.clone();

            tokio::spawn(async move {
                {
                    let mut stream = cons.stream();
                    loop {
                        let conn = conn.clone();
                        let producer = producer.clone();

                        match stream.next().await {
                            Some(Ok(msg)) => {
                                info!(?msg, "message received");

                                tokio::spawn(async move {
                                    events::process(
                                        msg,
                                        conn.clone(),
                                        producer.clone(),
                                        gift_amount,
                                    )
                                    .await
                                });
                                task::yield_now().await;
                            },
                            None => (),
                            Some(Err(e)) => {
                                warn!("failed to get message {:?}", e);
                            },
                        }
                    }
                }
            });

            Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
                .run(
                    Route::new()
                        .at(
                            "/graphql",
                            post(graphql_handler).with(AddData::new(state.clone())),
                        )
                        .at(
                            "/internal/organizations/:organization",
                            get(get_organization).data(connection),
                        )
                        .at("/playground", get(playground))
                        .at("/health", get(health)),
                )
                .await
                .context("failed to build graphql server")
        })
    });
}
