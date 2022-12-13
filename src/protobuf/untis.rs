/// This an implementation specific for my usecase with hardcoded keys.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Substitution {
    #[prost(string, tag = "1")]
    pub klasse: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub stunde: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub fach: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub fach_alt: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub raum: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub raum_alt: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub vertr_von: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub art: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plan {
    #[prost(string, tag = "1")]
    pub date: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub weekday: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub week_type: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub news: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub affected_classes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub content: ::prost::alloc::vec::Vec<Substitution>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Overview {
    #[prost(string, tag = "1")]
    pub plan_url: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub last_updated: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub current: ::core::option::Option<Plan>,
    #[prost(message, optional, tag = "4")]
    pub upcoming: ::core::option::Option<Plan>,
}
