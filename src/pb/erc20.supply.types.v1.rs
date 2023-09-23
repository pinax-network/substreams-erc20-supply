// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalSupplies {
    #[prost(message, repeated, tag="1")]
    pub total_supplies: ::prost::alloc::vec::Vec<TotalSupply>,
}
// message StorageKeys{
//    repeated StorageKey storage_keys = 1;
// }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalSupply {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// string transaction = 3;
    #[prost(string, tag="2")]
    pub supply: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
