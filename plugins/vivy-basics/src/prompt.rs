//! The default coding identity selected by the recipe.
use rig_harness::prelude::*;

/// Rig's default coding guidance with this product's identity.
#[derive(Default)]
pub struct VivyPromptPlugin;

impl Plugin for VivyPromptPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(AgentPass, default_prompt.in_set(PrepareSystems::Compose));
    }
}

fn default_prompt(
    mut calls: Query<&mut ModelRequest, (Added<Admitted>, Or<(With<TurnCall>, With<SideCall>)>)>,
) {
    for mut call in &mut calls {
        if let Some(prompt) = &mut call.prompt
            && prompt.role.is_empty()
        {
            prompt.role = rig_basics::prompt::DEFAULT_SYSTEM_PROMPT.replacen(
                "You are rig,",
                "You are vivy-test,",
                1,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rig_core::test_utils::MockCompletionModel;
    use rig_harness_test_support::{MemoryStore, app_on, connect, first_agent, reply, run_until};

    #[test]
    fn default_identity_reaches_the_model_without_replacing_a_custom_role() {
        let model = MockCompletionModel::from_stream_turns([reply("one"), reply("two")]);
        let (mut app, wakes) = app_on(&MemoryStore::default());
        app.add_plugins((AgentPlugin, SessionLogPlugin, VivyPromptPlugin));
        app.finish();
        app.update();
        let agent = first_agent(app.world_mut()).unwrap();
        app.world_mut()
            .entity_mut(agent)
            .insert(connect(&model).unwrap());
        app.world_mut()
            .trigger(Deliver::new(agent, "hello", DeliveryMode::Queue));
        run_until(&mut app, &wakes, |world| {
            !world.entity(agent).contains::<ActiveTurn>() && model.request_count() == 1
        });
        let first = format!("{:?}", model.requests()[0].chat_history);
        assert!(
            first.contains("You are vivy-test, a coding agent"),
            "{first}"
        );
        assert!(!first.contains("You are rig,"));

        app.world_mut()
            .entity_mut(agent)
            .insert(SystemPrompt("CUSTOM-OWNER-ROLE".into()));
        app.world_mut()
            .trigger(Deliver::new(agent, "again", DeliveryMode::Queue));
        run_until(&mut app, &wakes, |world| {
            !world.entity(agent).contains::<ActiveTurn>() && model.request_count() == 2
        });
        let second = format!("{:?}", model.requests()[1].chat_history);
        assert!(second.contains("CUSTOM-OWNER-ROLE"));
        assert!(!second.contains("You are vivy-test,"));
    }
}
