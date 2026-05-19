# opsd-rust

`opsd-rust` is the Rust client library for the Opsd API. It provides a small typed wrapper around
the current public endpoints, including the hello-world sandbox route and the user endpoints.

The default constructor, `OpsdClient::new()`, targets the production API at `https://api.opsd.sh/`.
For local development, tests, or non-production deployments, use `OpsdClient::new_base(url)` with a
parsed `url::Url`. Successful responses deserialize into typed Rust structs, and non-2xx responses
are surfaced as `Error::Api` with the underlying `ProblemDetails` attached.
