# Opsd Rust library

`opsd` is the Rust client library for the Opsd API. It provides a small typed
wrapper around the current public endpoints, including business, practice,
payroll and billing workflows and the existing sandbox endpoints.

The public `opsd::types` module owns the API-facing domain and operation types
independently of the optional HTTP client.

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

## Practices and billing

Practice methods follow the business API's membership and invitation pattern:
create/list/get practices, manage members, and create/list/cancel/accept/decline
invitations. `create_practice_business` creates a new client business without
direct members; it does not link an existing business. Practice roles are
`Admin` and `Member`.

Standalone businesses use `subscribe_to_payroll`, `get_payroll_subscription`
and `cancel_payroll_subscription`. Practice admins instead use
`enable_practice_payroll`, `get_practice_payroll` and `disable_practice_payroll`
with both practice and business IDs. Agreement status is separate from
suspension; enabling or subscribing does not clear suspension or itself start
billing.

Both payer types have billing-status, payment-method listing/default selection,
invoice listing, detailed invoice lines and PDF-download methods. Practice
variants include `practice` in their names, such as `list_practice_invoices`.
Billing operations require administrator access. Billing status and payment
methods come from Stripe live; invoice lists and breakdowns are saved Opsd data.
Selecting a default method does not retry invoices or clear suspension.

List methods take typed query structs; `Default::default()` uses server defaults.
Use `next_before`, `next_offset` or `next_starting_after` in the next request to
continue pagination. Invoice subtotals are not remaining balances. Practice
invoice lines identify each client business; the PDF contains summary lines.
PDF methods return `Vec<u8>` without writing files or opening a browser.

Website billing-portal sessions use browser cookies, not API credentials, and
are intentionally not included. All public models remain available with
`default-features = false` when only API types are needed.

## License

Licensed under either the Apache License, Version 2.0 or the MIT license, at
your option.
