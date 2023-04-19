//!

use holaplex_hub_credits::{
    build_schema,
    db::Connection,
    handlers::{get_organization, graphql_handler, health, playground},
    AppState, Args,
};
use hub_core::anyhow::Context as AnyhowContext;
use poem::{get, listener::TcpListener, middleware::AddData, post, EndpointExt, Route, Server};

pub fn main() {
    let opts = hub_core::StartConfig {
        service_name: "hub-credits",
    };

    hub_core::run(opts, |common, args| {
        let Args { port, db } = args;

        common.rt.block_on(async move {
            let connection = Connection::new(db)
                .await
                .context("failed to get database connection")?;

            let schema = build_schema();

            let state = AppState::new(schema, connection.clone());

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
