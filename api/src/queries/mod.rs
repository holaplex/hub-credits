#![allow(clippy::unused_async)] // async-graphql requires the async keyword

mod organization;

// Add your other ones here to create a unified Query object
#[derive(Debug, async_graphql::MergedObject, Default)]
pub struct Query(organization::Query);
