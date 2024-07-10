// @generated
/// A store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Store {
    /// A unique id (e.g. store number)
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An identifier indicating the type of store
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    /// Store name (human-readable)
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    /// Store location
    #[prost(message, optional, tag = "6")]
    pub location: ::core::option::Option<Location>,
    /// Store address
    #[prost(message, optional, tag = "7")]
    pub address: ::core::option::Option<Address>,
    /// Store hours
    #[prost(string, tag = "8")]
    pub store_hours: ::prost::alloc::string::String,
}
/// An address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    /// Street address
    #[prost(string, tag = "1")]
    pub street: ::prost::alloc::string::String,
    /// City
    #[prost(string, tag = "2")]
    pub city: ::prost::alloc::string::String,
    /// State
    #[prost(string, tag = "3")]
    pub state: ::prost::alloc::string::String,
    /// Zip code
    #[prost(int32, tag = "4")]
    pub zip_code: i32,
    /// Country
    #[prost(string, tag = "5")]
    pub region_code: ::prost::alloc::string::String,
    /// County
    #[prost(string, tag = "6")]
    pub county: ::prost::alloc::string::String,
}
/// A location in terrestrial coordinates.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Latitude of the location.
    #[prost(float, tag = "1")]
    pub latitude: f32,
    /// Longitude of the location.
    #[prost(float, tag = "2")]
    pub longitude: f32,
}
/// A bounding box in terrestrial coordinates.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingBox {
    /// Maximum coordinate values.
    #[prost(message, optional, tag = "1")]
    pub max: ::core::option::Option<Location>,
    /// Minimum coordinate values.
    #[prost(message, optional, tag = "2")]
    pub min: ::core::option::Option<Location>,
}
/// Request to FindStores.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindStoresRequest {
    /// Bounding box of the request.
    #[prost(message, optional, tag = "1")]
    pub bounds: ::core::option::Option<BoundingBox>,
    /// Maximum number of results to return.
    #[prost(int32, tag = "2")]
    pub limit: i32,
}
/// Response from FindStores.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindStoresResponse {
    /// Number of matching stores.
    #[prost(int32, tag = "1")]
    pub count: i32,
    /// Matching stores.
    #[prost(message, repeated, tag = "2")]
    pub stores: ::prost::alloc::vec::Vec<Store>,
}
/// Request to GetStore.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoreRequest {
    /// The ID of the store resource to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to ListStores.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStoresRequest {
    /// Page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Pagination token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from ListStores.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStoresResponse {
    /// List of stores.
    #[prost(message, repeated, tag = "1")]
    pub stores: ::prost::alloc::vec::Vec<Store>,
    /// Token of next page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
include!("bobadojo.stores.v1.tonic.rs");
include!("bobadojo.stores.v1.serde.rs");
// @@protoc_insertion_point(module)
