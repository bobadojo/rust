// @generated
/// Request to GetMenu.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMenuRequest {
    /// The ID of the menu resource to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A store resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Menu {
    /// A unique id.
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// A description of the menu
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
include!("bobadojo.menus.v1.tonic.rs");
include!("bobadojo.menus.v1.serde.rs");
// @@protoc_insertion_point(module)
