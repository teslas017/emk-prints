# EMK PRINTS API

Headless Rust/Actix Web API for products, inventory, orders, administration, media metadata and hosted-payment webhooks.

## Security baseline

- TLS is terminated by the hosting platform; HTTP is redirected to HTTPS.
- PostgreSQL queries are parameterized through SQLx.
- Request bodies, uploads and field lengths are bounded and validated.
- Admin passwords use Argon2id; MFA and short-lived secure-cookie sessions are required before production.
- Payment details are handled only by the selected hosted provider. Webhook signatures and idempotency keys must be verified before enabling checkout.
- Product media belongs in private object storage with validated MIME signatures, randomized keys and signed upload URLs.
- Secrets stay in the host secret manager, never source control.
- Rate limiting, restrictive CORS, CSRF protection, audit logs, encrypted backups and monitoring are required production controls.

The payment and admin authentication handlers intentionally fail closed until the provider and first administrator are configured.
