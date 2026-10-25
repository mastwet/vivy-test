//! Rig's skills, gated by project trust: the same discovery, listing and
//! `/skill:<name>` commands as `rig_basics::skills::SkillsPlugin`, which it
//! replaces in the recipe, without the project's skill directories until
//! the project is trusted ([`crate::trust`]).

use bevy_log::warn;
use rig_basics::skills::{Skill, discover, message, roots};
use rig_harness::prelude::*;

/// Skills from the trusted places only.
#[derive(Default)]
pub struct TrustedSkillsPlugin;

impl Plugin for TrustedSkillsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectRoot>();
    }

    fn finish(&self, app: &mut App) {
        let world = app.world_mut();
        let home = RigHome::of(world).root().to_path_buf();
        let root = world.resource::<ProjectRoot>().0.clone();
        let places: Vec<_> = roots(&root, &home)
            .into_iter()
            .filter(|place| crate::trust::allowed(world, place))
            .collect();
        let (skills, problems) = discover(&places);
        for problem in problems {
            warn!("{problem}");
        }
        world.spawn((
            Name::new("prompt:skills"),
            PromptSection::new(
                PromptSection::ORDER_PROJECT + 10,
                "skills",
                listing(&skills),
            ),
        ));
        for skill in skills {
            let help = format!("Skill: {}", first_line(&skill.description));
            let name = format!("skill:{}", skill.name);
            app.add_command(
                &name,
                &help,
                move |In(args): In<CommandArgs>,
                      mut commands: Commands,
                      mut notices: MessageWriter<Notice>| {
                    match message(&skill, &args.args) {
                        Ok(text) => {
                            let author = Author::user().titled(format!("skill {}", skill.name));
                            commands.spawn((
                                Incoming::new(text, args.mode).with_author(author),
                                ForAgent(args.agent),
                            ));
                        }
                        Err(why) => {
                            notices.write(Notice::error(
                                args.agent,
                                format!("Cannot read {}: {why}.", skill.file.display()),
                            ));
                        }
                    }
                },
            );
        }
    }
}

/// The prompt section listing the skills the model may pick, as Rig's.
fn listing(skills: &[Skill]) -> String {
    let entries: Vec<String> = skills
        .iter()
        .filter(|skill| !skill.manual)
        .map(|skill| {
            format!(
                "  <skill>\n    <name>{}</name>\n    <description>{}</description>\n    \
                 <location>{}</location>\n  </skill>",
                escape(&skill.name),
                escape(&skill.description),
                escape(&skill.file.display().to_string()),
            )
        })
        .collect();
    if entries.is_empty() {
        return String::new();
    }
    format!(
        "Skills give instructions for specific tasks. When a task matches a skill's \
         description, read the skill's file with the read tool before you start, and follow \
         it. A relative path in a skill is relative to the skill's directory, the directory \
         of its file.\n\n<available_skills>\n{}\n</available_skills>",
        entries.join("\n")
    )
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn first_line(text: &str) -> String {
    let line = text.lines().next().unwrap_or_default();
    match line.char_indices().nth(80) {
        Some((at, _)) => format!("{}…", &line[..at]),
        None => line.to_owned(),
    }
}
