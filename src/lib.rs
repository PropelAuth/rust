//! # PropelAuth
//!
//! Add authentication and authorization to your application.
//!
//! This library is meant to be used with a [PropelAuth](https://www.propelauth.com/) account.
//! You can sign up and get started for free.
//!
//! # Initialize
//!
//! First, you'll need to initialize the library. You can either call `PropelAuth::init`
//! or `PropelAuth::fetch_and_init` (which will fetch any unspecified metadata).
//!
//! ```rust
//! let auth = PropelAuth::fetch_and_init(AuthOptions {
//!     auth_url: "REPLACE_ME".to_string(),
//!     api_key: "REPLACE_ME".to_string(),
//! }).await.expect("Unable to initialize authentication");
//! ```
//!
//! # Usage / Protecting APIs
//!
//! - [Axum](#axum)
//! - [Actix](#actix)
//! - [Other](#other)
//!
//! Want us to add support for another framework? Reach out at support@propelauth.com
//!
//! ## Axum
//!
//! To use Axum, make sure to enable the `axum_07` (or `axum_06`) feature in your Cargo.toml.
//!
//! Then, add PropelAuthLayer to your Router:
//!
//! ```rust
//! let auth_layer = PropelAuthLayer::new(auth);
//!
//! let app = Router::new()
//!     .route("/whoami", get(whoami))
//!     .route("/org/:org_name/whoami", get(org_whoami))
//!     .layer(auth_layer); // <-- here
//! ```
//!
//! You can then take `User` in as an argument, which will look for an [access token](https://docs.propelauth.com/overview/access-token/) in the Authorization header.
//!
//! ```rust
//! // User will automatically return a 401 (Unauthorized) if a valid access token wasn't provided
//! async fn whoami(user: User) -> String {
//!     user.user_id
//! }
//! ```
//!
//! You can also check which [organizations](https://docs.propelauth.com/overview/organizations/) the user is in, and which [roles and permissions](https://docs.propelauth.com/overview/rbac/) they have.
//!
//! ```rust
//! // If the user isn't in the provided organization, a 403 is returned
//! async fn org_whoami(user: User,
//!                     Path(org_name): Path<String>) -> Result<String, UnauthorizedOrForbiddenError> {
//!     let org = user.validate_org_membership(RequiredOrg::OrgName(&org_name),
//!                                            UserRequirementsInOrg::IsRole("Admin"))?;
//!     Ok(format!("You are a {} in {}", org.user_role, org.org_name))
//! }
//! ```
//!
//! You can also get the full `auth` struct and make API calls with it:
//!
//! ```rust
//! // Extension(auth) is useful for making API requests
//! async fn make_req(Extension(auth): Extension<Arc<PropelAuth>>) -> String {
//!     let magic_link = auth.user().create_magic_link(CreateMagicLinkRequest {
//!         email: "user@customer.com".to_string(),
//!         ..Default::default()
//!     }).await.expect("Couldn't create magic link");
//!     magic_link.url
//! }
//! ```
//! ## Actix
//!
//! To use Actix, make sure to enable the `actix4` feature in your Cargo.toml.
//!
//! Add your PropelAuth to your Router:
//!
//! ```rust
//! let auth = PropelAuth::fetch_and_init(/*...*/)
//! //...
//! HttpServer::new(move || {
//!     App::new()
//!         .service(whoami)
//!         .service(org_whoami)
//!         .app_data(web::Data::new(auth.clone())) // <-- here
//! })
//! ```
//!
//! You can then take `User` in as an argument, which will look for an [access token](https://docs.propelauth.com/overview/access-token/) in the Authorization header.
//!
//! ```rust
//! // User will automatically return a 401 (Unauthorized) if a valid access token wasn't provided
//! #[get("/whoami")]
//! async fn whoami(user: User) -> impl Responder {
//!     HttpResponse::Ok().json(user)
//! }
//! ```
//!
//! You can also check which [organizations](https://docs.propelauth.com/overview/organizations/) the user is in, and which [roles and permissions](https://docs.propelauth.com/overview/rbac/) they have.
//!
//! ```rust
//! // If the user isn't in the provided organization, a 403 is returned
//! #[get("/org/{org_name}/whoami")]
//! async fn whoami(user: User, org_name: web::Path<String>) -> Result<impl Responder, UnauthorizedOrForbiddenError> {
//!    let org = user.validate_org_membership(RequiredOrg::OrgName(&org_name.into_inner()),
//!                                           UserRequirementsInOrg::IsRole("Admin"))?;
//!    Ok(HttpResponse::Ok()
//!        .body(format!("You are a {} in {}", org.user_role, org.org_name)))
//! }
//! ```
//!
//! You can also get the full `auth` struct and make API calls with it:
//!
//! ```rust
//! #[post("/magic_link")]
//! async fn make_req(auth: web::Data<PropelAuth>) -> impl Responder {
//!     let magic_link = auth.user().create_magic_link(CreateMagicLinkRequest {
//!         email: "user@customer.com".to_string(),
//!         ..Default::default()
//!     }).await.expect("Couldn't create magic link");
//!     HttpResponse::Ok().json(magic_link)
//! }
//! ```
//!
//! ## Other
//!
//! After initializing `auth`, you can verify [access tokens](https://docs.propelauth.com/overview/access-token/) by passing in the Authorization header (formatted `Bearer TOKEN`):
//!
//! ```rust
//! let result = auth.verify().validate_authorization_header(&authorization_header);
//! match result {
//!     Ok(user) => { /* valid access token in the header */ }
//!     Err(_) => { /* invalid access token, typically we return a 401 Unauthorized here */ }
//! }
//! ```
//! You can also check which [organizations](https://docs.propelauth.com/overview/organizations/) the user is in, and which [roles and permissions](https://docs.propelauth.com/overview/rbac/) they have.
//!
//! ```rust
//! let org = auth.validate_org_membership(
//!     &authorization_header,
//!     RequiredOrg::OrgName("acme"),
//!     UserRequirementsInOrg::IsRole("Admin")
//! )?;
//!
//! // Alternatively, if you already have a user from validate_authorization_header
//! let org = user.validate_org_membership(
//!     RequiredOrg::OrgName("acme"),
//!     UserRequirementsInOrg::IsRole("Admin")
//! )?;
//! ```
//!
//! And finally, you can make API calls directly from `auth.user()` and `auth.org()`
//!
//! # Where do the access tokens come from?
//!
//! They come from your frontend.
//! You can read more about integrating your frontend [here](https://docs.propelauth.com/getting-started/frontend-integration/).

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;
pub mod propelauth;

#[cfg(feature = "axum06")]
pub mod axum06;

#[cfg(feature = "axum07")]
pub mod axum07;

#[cfg(feature = "actix4")]
pub mod actix;

#[cfg(feature = "tonic011")]
pub mod tonic011;
#[cfg(feature = "tonic012")]
pub mod tonic012;
