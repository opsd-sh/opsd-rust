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
suspension. Enabling or subscribing starts monthly base fees without clearing
suspension. Mutations take `PayrollSubscriptionAction` with `expected_version`
from the status response’s `subscription_version`.

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

## Payroll calculations

Scheme creation requires `tax_year_start` and an explicit `EmploymentAllowance`
decision. Create a monthly schedule with `create_pay_schedule` before creating
employments or draft runs; both requests require its `PayScheduleId`.

New employments require a start date, NI category, monthly gross pay and
`StarterInformation` (`P45`, `Checklist` or `NotProvided`), a
`NormalWeeklyHoursBand`, and explicit `StudentLoan::NotApplicable` and
`PostgraduateLoan::NotApplicable` declarations. Loan deductions are not yet
supported; do not use these declarations where deductions apply. P45 tax details and
HMRC notices use cumulative/non-cumulative enum variants so irrelevant amounts
cannot accompany W1/M1 treatment. Amounts are sterling pence; tax amounts are
signed to preserve refunds. Tax codes and year-specific HMRC rules are validated
by the server; this public library has no dependency on HMRC calculation crates.

`get_employment` returns the original starter evidence and active-year tax
settings. `update_employment` accepts `UpdateEmploymentRequest::Settings` or
`::TaxCodeNotice`; notice updates cannot also contain starter changes. For a
cumulative notice, omitted earlier-job amounts retain stored cumulative totals.
Switching from non-cumulative treatment requires explicit amounts, including
zero where appropriate. These amounts are separate from this employment's YTD.

`preview_payroll_run` calculates a draft with current settings and saved gross
inputs. `update_payroll_payment` overrides one draft payment in pence, and
`discard_payroll_run` deletes a draft. Finalisation stores the calculation;
`get_payroll_run` and `finalize_payroll_run` retain its payments, inputs, YTD and
totals in `details.calculation`. Drafts and historical runs without saved results
have `None`. Subsequent notices do not rewrite saved results.

`rollover_paye_scheme` takes the destination year, Employment Allowance decision
and an explicit list of new-year notices (`[]` after checking none apply).
The rollover contract includes opserver sending an end-of-year FPS for the
closing year and an EPS declaring the Employment Allowance decision for the new
year, alongside closing the old year and preparing new-year employment settings.
Use `list_paye_scheme_rollovers` to recover a rollover ID after a lost response,
then `get_paye_scheme_rollover` to inspect progress and `available_actions`.
Use the explicit retry, resume or cancel methods with the saved version;
repeating the start request is not a resume operation. The response reports
separate final-FPS and new-year-EPS progress. The server currently supports
2026–27 calculations and rollover rules; entering 2027–28 is not yet supported.

These changes add required fields to the creation requests. Release the updated
SDK before updating consumers that use the published crate. All payroll models
remain available with `default-features = false`.

## Current API changes in 0.4

Employee creation requires date of birth, sex (`M` or `F`) and an address, in
addition to names. Title, passport number and National Insurance number are
optional. `UpdateEmployeeRequest` uses `FieldUpdate::Unchanged`, `::Set(value)`
and `::Clear` for these nullable fields. Other omitted update fields retain
their values. The server enforces HMRC validation and payroll-history edit rules.

Practice creation requires `PracticeReportingDetails`. Read or replace them with
`get_practice_hmrc_reporting` and `update_practice_hmrc_reporting`. Scheme creation
also accepts an optional Corporation Tax `company_utr`.

`finalize_payroll_run` takes `FinalizePayrollRunRequest`: the run's current
`version`, Government Gateway sender ID and `HmrcPassword`, plus an optional
late-reporting reason. Successful HTTP completion does not mean HMRC acceptance.
`PayrollRunStatus::Finalized` carries submission progress; `::Accepted` establishes
acceptance. Read `available_actions` before retrying, reopening, cancelling or
resuming. Resuming polling requires only the version; reconciliation also needs
the original sender's password. The client never retries these actions automatically.

Rollover creation additionally requires `expected_generation` from the scheme's
`rollover_generation`. Subsequent rollover actions take its own `version`.
History methods return revisions, attempts, actions and safe diagnostic messages.
An API conflict with an HMRC waiting period exposes `Error::retry_after_seconds()`.
Re-read progress after a conflict or uncertain transport result; do not silently
substitute a newer version and repeat an action.

`HmrcPassword` redacts Debug output, but serialization exposes its value for the
request. Do not log serialized credential-bearing requests. Credentials are not
returned in progress or history responses. Automatic HTTP redirects are disabled
to prevent forwarding credential-bearing request bodies; redirects are returned
as errors for the caller to investigate.

## License

Licensed under either the Apache License, Version 2.0 or the MIT license, at
your option.
