# Opsd Rust library

`opsd` is the Rust client library for the Opsd API. It provides a small
typed wrapper around the current public endpoints, including the hello-world
sandbox route and the user endpoints.

The default constructor, `OpsdClient::new()`, targets the production API at
`https://api.opsd.sh/v1/`.
For local development, tests, or non-production deployments, use
`OpsdClient::new_base(url, credential)` with a parsed `url::Url`. Successful
responses deserialize into typed Rust structs, and non-2xx responses are
surfaced as `Error::Api` with the underlying `ProblemDetails` attached.

Clients require an API credential. OAuth access tokens and API keys both use
the HTTP Bearer scheme without exposing the secret in debug output:

```rust
let credential = ApiCredential::new(secret)?;
let client = OpsdClient::new(credential)?;
```

## License

Licensed under either the Apache License, Version 2.0 or the MIT license, at
your option.
