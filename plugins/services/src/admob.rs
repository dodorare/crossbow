
extern crate google_admob1 as admob1;
use admob1::{AdMob, oauth2};

pub async fn google_admob() -> crate::error::Result<()> {
// Get an ApplicationSecret instance by some means. It contains the `client_id` and
// `client_secret`, among other things.
let secret: oauth2::ApplicationSecret = Default::default();

// Instantiate the authenticator. It will choose a suitable authentication flow for you,
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await?;

let https = hyper_rustls::HttpsConnectorBuilder::new()
    .with_native_roots()
    .https_only()
    .enable_http1()
    .build();

let hub = AdMob::new(hyper::Client::builder().build(https), auth);

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
hub.accounts().get("name")
             .doit().await?;

Ok(())

}
