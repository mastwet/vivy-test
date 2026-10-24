//! The model of one OpenAI-compatible gateway, as `gateway/<model>`.
//!
//! `RIG_HOME/settings.toml`:
//!
//! ```toml
//! [gateway]
//! base_url = "https://gateway.example/v1"
//! model = "provider/model-id"   # sent to the gateway unchanged
//! ```
//!
//! The key is the agent's secret `gateway`, the variable
//! `RIG_SECRET_GATEWAY`. The harness passes secrets to no process it starts,
//! so the shell and other tools never see it. The model is chosen with
//! `--model gateway/<model>` or `/model`. Requests go to Chat Completions,
//! the endpoint OpenAI-compatible gateways share.
//!
//! Only the user's settings are read, never a project's `.rig/settings.toml`,
//! so a cloned repository cannot send the key elsewhere.

use bevy_log::warn;
use rig_harness::prelude::*;
use rig_harness::rig_core::{
    catalog::{Catalog, ModelFacts, ModelSpec, Transport},
    operation::Completion,
    providers::openai::{OpenAIConfig, Route, wire::Dialect},
    providers::registry::{ConnectError, ProviderConfig, ProviderId},
    serve::{ErasedHandler, adapters::ModelAdapter},
};
use serde::Deserialize;

/// The vendor the gateway's model is listed under.
pub const VENDOR: &str = "gateway";
/// The secret that holds the gateway's key.
pub const SECRET: &str = "gateway";

/// OpenAI's wire with the quirks every compatible gateway shares; the base
/// URL and key come from the settings and the secret.
static DIALECT: Dialect = Dialect::gateway(VENDOR, "", "");

/// `[gateway]` in `RIG_HOME/settings.toml`.
#[derive(Resource, Deserialize, Default, Clone, Debug, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct GatewaySettings {
    /// The gateway's OpenAI-compatible base URL, such as `https://host/v1`.
    pub base_url: String,
    /// The model id the gateway serves, sent as given.
    pub model: String,
}

impl GatewaySettings {
    /// What is missing, or `None` when the gateway can be listed.
    fn missing(&self) -> Option<&'static str> {
        if self.base_url.trim().is_empty() {
            Some("base_url")
        } else if self.model.trim().is_empty() {
            Some("model")
        } else {
            None
        }
    }
}

/// Lists the configured model as `gateway/<model>` and reaches it through
/// the gateway with the `gateway` secret.
#[derive(Default)]
pub struct GatewayPlugin;

impl Plugin for GatewayPlugin {
    fn build(&self, app: &mut App) {
        app.add_secret(Secret::new(SECRET, "API key of the model gateway"));
        app.add_setting::<GatewaySettings>("gateway");
        let settings = app.world().resource::<GatewaySettings>().clone();
        if let Some(missing) = settings.missing() {
            warn!("gateway: settings.toml has no [gateway] {missing}; no gateway model is listed");
            return;
        }
        let key = app
            .world()
            .get_resource::<Secrets>()
            .and_then(|secrets| secrets.get(SECRET))
            .filter(|key| !key.is_empty())
            .map(str::to_owned);
        let vendor = match ProviderId::declare(VENDOR) {
            Ok(vendor) => vendor,
            Err(why) => return warn!("gateway: cannot declare the `{VENDOR}` vendor: {why}"),
        };
        let mut models = app.world_mut().get_resource_or_init::<Models>();
        let spec = ModelSpec::new(vendor, settings.model.trim()).with_tools(true);
        models.0.catalog_mut().insert(spec);
        let base_url = settings.base_url.trim().to_owned();
        models.0.provide(
            vendor,
            move |spec: &ModelSpec, _: &Catalog, transport: &Transport| {
                connect(&base_url, key.as_deref(), spec, transport)
            },
        );
        app.add_systems(PostStartup, default_gateway_model);
    }
}

fn default_gateway_model(
    settings: Res<GatewaySettings>,
    invoked: Option<Res<Invoked>>,
    agents: Query<Entity, (With<Agent>, Without<ModelChoice>, Without<SpawnedBy>)>,
    mut commands: Commands,
) {
    if invoked.is_some_and(|invoked| invoked.args.model.is_some()) {
        return;
    }
    for agent in &agents {
        commands
            .entity(agent)
            .insert(ModelChoice(format!("gateway/{}", settings.model.trim())));
    }
}

/// `spec` on the gateway at `base_url`, or why it cannot be reached.
fn connect(
    base_url: &str,
    key: Option<&str>,
    spec: &ModelSpec,
    transport: &Transport,
) -> Result<ErasedHandler, ConnectError> {
    let key = key.ok_or_else(|| ConnectError::MissingKey {
        vendor: VENDOR.to_owned(),
        tried: vec![rig_harness::harness_protocol::secret_variable(SECRET).into()],
    })?;
    let http = transport.shared().ok_or(ConnectError::NoHttpClient)?;
    let config = OpenAIConfig::with_key(&DIALECT, key)
        .with_base_url(base_url)
        .with_route(Route::Chat);
    let model = ProviderConfig::OpenAi(config).completion_model(
        &spec.id,
        http,
        ModelFacts::new(spec.clone()),
    );
    let adapter = ModelAdapter::<Completion>::new(spec.reference(), model);
    Ok(ErasedHandler::new(adapter))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app(settings: &str, secrets: &[(&str, &str)]) -> App {
        let mut app = App::new();
        app.insert_resource(Settings::from_toml(settings));
        app.insert_resource(secrets.iter().copied().collect::<Secrets>());
        app.init_resource::<Models>();
        app.add_plugins(GatewayPlugin);
        app
    }

    const SETTINGS: &str =
        "[gateway]\nbase_url = \"http://127.0.0.1:9/v1\"\nmodel = \"org/model-1\"\n";

    #[test]
    fn lists_the_model_and_connects_with_the_secret() {
        let app = app(SETTINGS, &[(SECRET, "test-key")]);
        let models = app.world().resource::<Models>();
        let spec = models.0.catalog().resolve("gateway/org/model-1").unwrap();
        assert!(spec.shared().tools);
        assert!(models.connect("gateway/org/model-1").is_ok());
    }

    #[test]
    fn names_the_secret_variable_when_the_key_is_missing() {
        let app = app(SETTINGS, &[]);
        let models = app.world().resource::<Models>();
        let Err(why) = models.connect("gateway/org/model-1") else {
            panic!("connected without a key");
        };
        assert!(why.contains("RIG_SECRET_GATEWAY"), "{why}");
    }

    #[test]
    fn lists_nothing_without_settings() {
        let app = app("", &[(SECRET, "test-key")]);
        let models = app.world().resource::<Models>();
        assert!(models.0.catalog().resolve("gateway/org/model-1").is_err());
    }

    #[test]
    fn configured_gateway_is_the_default_but_does_not_replace_restored_or_explicit_models() {
        let mut app = app(SETTINGS, &[(SECRET, "test-key")]);
        let fresh = app.world_mut().spawn(Agent).id();
        let restored = app
            .world_mut()
            .spawn((Agent, ModelChoice("gateway/remembered".into())))
            .id();
        app.update();
        assert_eq!(
            app.world().get::<ModelChoice>(fresh).unwrap().0,
            "gateway/org/model-1"
        );
        assert_eq!(
            app.world().get::<ModelChoice>(restored).unwrap().0,
            "gateway/remembered"
        );

        let mut app = super::tests::app(SETTINGS, &[(SECRET, "test-key")]);
        let mut invoked = Invoked::default();
        invoked.args.model = Some("gateway/explicit".into());
        app.insert_resource(invoked);
        let fresh = app.world_mut().spawn(Agent).id();
        app.update();
        assert!(app.world().get::<ModelChoice>(fresh).is_none());
    }
}
