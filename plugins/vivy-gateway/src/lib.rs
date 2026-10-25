//! The model of one OpenAI-compatible gateway, as `gateway/<model>`.
//!
//! `RIG_HOME/settings.toml`:
//!
//! ```toml
//! [gateway]
//! base_url = "https://gateway.example/v1"
//! model = "provider/model-id"   # sent to the gateway unchanged
//! # Optional facts about the model, which Rig acts on:
//! dialect = "openrouter"         # a Rig OpenAI-compatible dialect: its option mapping
//! context_window = 131072        # compaction keeps requests inside it
//! max_output_tokens = 8192
//! image_input = true             # images in messages and `read` reach the model
//! reasoning = ["low", "medium", "high"]  # `/effort` levels, with a `dialect`
//! input_price = 0.5              # USD per million tokens, both or neither
//! output_price = 1.5
//! ```
//!
//! Without `dialect`, request options such as the reasoning level are
//! refused, because Rig maps them per dialect and knows none for an
//! unnamed gateway; the gateway's own dialect name (such as `openrouter`)
//! selects Rig's mapping for it.
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
    catalog::{Catalog, ModelFacts, ModelSpec, Pricing, ReasoningSupport, Transport},
    completion::Effort,
    operation::Completion,
    providers::openai::{OpenAIConfig, Route, wire},
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
static DIALECT: wire::Dialect = wire::Dialect::gateway(VENDOR, "", "");

/// `[gateway]` in `RIG_HOME/settings.toml`.
#[derive(Resource, Deserialize, Default, Clone, Debug, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct GatewaySettings {
    /// The gateway's OpenAI-compatible base URL, such as `https://host/v1`.
    pub base_url: String,
    /// The model id the gateway serves, sent as given.
    pub model: String,
    /// The Rig OpenAI-compatible dialect whose quirks and option mapping
    /// the gateway takes, such as `openrouter`; none for the generic one.
    pub dialect: Option<String>,
    /// The most tokens the model reads and writes in one request.
    pub context_window: Option<u32>,
    /// The most tokens the model writes in one reply.
    pub max_output_tokens: Option<u32>,
    /// Whether the model reads images.
    pub image_input: bool,
    /// The reasoning levels the model takes (`minimal`, `low`, `medium`,
    /// `high`, `xhigh`, `max`, and `off` when reasoning can be turned off),
    /// which `/effort` offers; empty for a model that does not reason.
    pub reasoning: Vec<String>,
    /// USD per million input tokens; used together with `output_price`.
    pub input_price: Option<f64>,
    /// USD per million output tokens.
    pub output_price: Option<f64>,
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

    /// The configured dialect, or why it is not used.
    fn dialect(&self) -> Result<&'static wire::Dialect, String> {
        let Some(name) = self.dialect.as_deref().map(str::trim) else {
            return Ok(&DIALECT);
        };
        wire::by_name(name).ok_or_else(|| {
            let names: Vec<&str> = wire::all().map(|dialect| dialect.name).collect();
            format!("dialect `{name}` is not one of Rig's: {}", names.join(", "))
        })
    }
}

/// Every effort level, by the name `/effort` takes.
const EFFORTS: [Effort; 6] = [
    Effort::Minimal,
    Effort::Low,
    Effort::Medium,
    Effort::High,
    Effort::XHigh,
    Effort::Max,
];

/// The catalog row of the gateway's model: `settings`' facts on Rig's
/// defaults (text in, tools, nothing else known).
pub fn spec(vendor: ProviderId, settings: &GatewaySettings) -> ModelSpec {
    let mut spec = ModelSpec::new(vendor, settings.model.trim()).with_tools(true);
    spec.context_window = settings.context_window;
    spec.max_output_tokens = settings.max_output_tokens;
    spec.input.image = settings.image_input;
    // Rig sends a reasoning level only through a named dialect's mapping;
    // without one every call that carries a level would be refused.
    let mapped = settings
        .dialect()
        .is_ok_and(|dialect| dialect.name != VENDOR);
    if !settings.reasoning.is_empty() && !mapped {
        warn!("gateway: `reasoning` needs a `dialect` Rig maps it for; /effort is off");
    } else if !settings.reasoning.is_empty() {
        let mut levels = Vec::new();
        let mut can_disable = false;
        for name in &settings.reasoning {
            match name.trim() {
                "off" => can_disable = true,
                name => match EFFORTS.iter().find(|effort| effort.as_str() == name) {
                    Some(effort) => levels.push(*effort),
                    None => warn!(
                        "gateway: reasoning level `{name}` is not one of off, minimal, low, medium, high, xhigh, max"
                    ),
                },
            }
        }
        spec.reasoning = ReasoningSupport::Listed {
            levels,
            budget: None,
            can_disable,
            default: None,
        };
    }
    match (settings.input_price, settings.output_price) {
        (Some(input), Some(output)) => spec.pricing = Some(Pricing::new(input, output)),
        (None, None) => {}
        _ => {
            warn!("gateway: set both input_price and output_price, or neither; no prices are used")
        }
    }
    spec
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
        let dialect = settings.dialect().unwrap_or_else(|why| {
            warn!("gateway: {why}; using the generic dialect");
            &DIALECT
        });
        let mut models = app.world_mut().get_resource_or_init::<Models>();
        models.0.catalog_mut().insert(spec(vendor, &settings));
        let base_url = settings.base_url.trim().to_owned();
        models.0.provide(
            vendor,
            move |spec: &ModelSpec, _: &Catalog, transport: &Transport| {
                connect(dialect, &base_url, key.as_deref(), spec, transport)
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
    dialect: &'static wire::Dialect,
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
    let config = OpenAIConfig::with_key(dialect, key)
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

    fn settings(text: &str) -> GatewaySettings {
        let mut app = App::new();
        app.insert_resource(Settings::from_toml(text));
        app.add_setting::<GatewaySettings>("gateway");
        app.world().resource::<GatewaySettings>().clone()
    }

    #[test]
    fn facts_reach_the_catalog_row() {
        let text = format!(
            "{SETTINGS}context_window = 4000\nmax_output_tokens = 500\nimage_input = true\n\
             dialect = \"openrouter\"\nreasoning = [\"off\", \"low\", \"high\", \"bogus\"]\ninput_price = 0.5\noutput_price = 1.5\n"
        );
        let settings = settings(&text);
        let row = spec(ProviderId::declare(VENDOR).unwrap(), &settings);
        assert_eq!(row.context_window, Some(4000));
        assert_eq!(row.max_output_tokens, Some(500));
        assert!(row.input.image && row.input.text && row.tools);
        assert_eq!(
            row.reasoning,
            ReasoningSupport::Listed {
                levels: vec![Effort::Low, Effort::High],
                budget: None,
                can_disable: true,
                default: None,
            }
        );
        let names: Vec<_> = row.reasoning.choices().iter().map(|c| c.name).collect();
        assert_eq!(names, ["default", "off", "low", "high"]);
        assert_eq!(row.pricing, Some(Pricing::new(0.5, 1.5)));
    }

    #[test]
    fn without_facts_the_row_is_rigs_default() {
        let row = spec(ProviderId::declare(VENDOR).unwrap(), &settings(SETTINGS));
        assert_eq!((row.context_window, row.max_output_tokens), (None, None));
        assert!(!row.input.image);
        assert_eq!(row.reasoning, ReasoningSupport::None);
        let text = format!("{SETTINGS}reasoning = [\"high\"]\n");
        let row = spec(ProviderId::declare(VENDOR).unwrap(), &settings(&text));
        assert_eq!(row.reasoning, ReasoningSupport::None, "no dialect maps it");
        let text = format!("{SETTINGS}input_price = 0.5\n");
        assert_eq!(
            spec(ProviderId::declare(VENDOR).unwrap(), &settings(&text)).pricing,
            None
        );
    }

    #[test]
    fn dialect_names_a_rig_dialect() {
        assert_eq!(settings(SETTINGS).dialect().unwrap().name, VENDOR);
        let named = settings(&format!("{SETTINGS}dialect = \"openrouter\"\n"));
        assert_eq!(named.dialect().unwrap().name, "openrouter");
        let wrong = settings(&format!("{SETTINGS}dialect = \"nope\"\n"));
        let why = wrong.dialect().unwrap_err();
        assert!(
            why.contains("`nope`") && why.contains("openrouter"),
            "{why}"
        );
    }

    #[test]
    fn a_misspelled_key_is_not_a_silent_default() {
        let text = format!("{SETTINGS}context_windw = 4000\n");
        assert_eq!(settings(&text), GatewaySettings::default());
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
