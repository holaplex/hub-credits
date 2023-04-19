// Add your other ones here to create a unified Mutation object
// e.x. Mutation(OrganizationMutation, OtherMutation, OtherOtherMutation)
#[derive(Debug, async_graphql::MergedObject, Default)]
pub struct Mutation();
