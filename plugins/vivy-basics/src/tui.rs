//! The recipe-selected terminal presentation, on Rig's public widgets.
//! The native editor, scrolling, questions and session lifecycle remain owners
//! of their behavior. This plugin composes their screen and changes its looks.
use std::sync::Arc;

use bevy_ecs::message::MessageMutator;
use rig_harness::prelude::*;
use rig_tui::ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};
use rig_tui::{
    AsideRenderer, Editor, InlineStrip, InputContext, KeyFocus, MainEditor, MainPane, PaneKeys,
    PanelCanvas, Placement, RequestRedraw, StatusItems, StatusLine, StatusSystems, TerminalSize,
    TerminalWrite, Theme, TranscriptPane, TranscriptSource, TuiPanel, TuiSystems, TuiViewPlugin,
};

mod looks;

/// A compact coding screen with vivy-test branding and a persistent footer.
#[derive(Default)]
pub struct VivyTuiPlugin;

impl Plugin for VivyTuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TuiViewPlugin, rig_basics::welcome::WelcomePlugin));
        let world = app.world_mut();
        world.spawn((
            Name::new("vivy-test:footer"),
            StatusLine::default(),
            TuiPanel::new(Placement::BelowInput(Constraint::Length(1))),
        ));
        world.spawn((
            Name::new("vivy-test:input"),
            Editor::default(),
            MainEditor,
            TuiPanel::new(Placement::Bottom(Constraint::Length(3))),
        ));
        world.spawn((Name::new("vivy-test:questions"), InlineStrip));
        world.spawn((
            Name::new("vivy-test:transcript"),
            TranscriptPane::new(TranscriptSource::Shown),
            MainPane,
            PaneKeys,
        ));
        app.add_systems(PostUpdate, prioritize_status.in_set(StatusSystems::Edit))
            .add_systems(PostUpdate, prepare.in_set(TuiSystems::Prepare))
            // The native widget has already drawn its text and caret. Change
            // only its chrome before composition; overlays still draw last.
            .add_systems(
                PostUpdate,
                input_chrome
                    .after(TuiSystems::Draw)
                    .before(TuiSystems::Render),
            )
            // Rig sends its state in PostUpdate and writes in Last, after
            // OnAppExitSystems. Its exit-time title pop passes through intact.
            .add_systems(Last, brand_title.before(OnAppExitSystems));
    }

    fn finish(&self, app: &mut App) {
        looks::install(app);
    }
}

/// Keep the same terminal-derived palette, with quieter neutral surfaces.
/// Compare before writing so an idle app does not redraw indefinitely.
fn prepare(
    mut theme: ResMut<Theme>,
    size: Res<TerminalSize>,
    mut width: Local<Option<u16>>,
    mut asides: Query<&mut AsideRenderer>,
    mut redraw: MessageWriter<RequestRedraw>,
) {
    let mut next = *theme;
    next.user = theme.shade(0.06);
    next.input = theme.shade(0.08);
    next.tool = Color::Reset;
    next.call_done = Color::Reset;
    next.call_waiting = Color::Reset;
    next.call_running = theme.shade(0.04);
    if *theme != next {
        *theme = next;
        redraw.write(RequestRedraw);
    }
    let columns = size.0.width;
    if *width != Some(columns) {
        *width = Some(columns);
        for mut renderer in &mut asides {
            if renderer.kind == rig_basics::welcome::WELCOME {
                renderer.render = Arc::new(move |view| looks::welcome(view, columns));
            }
        }
    }
}

/// Preserve the data supplied by plugins, changing only fitting priorities.
fn prioritize_status(mut statuses: Query<&mut StatusItems>) {
    for mut items in &mut statuses {
        let mut next = items.clone();
        for mut item in items.iter().cloned() {
            match item.name {
                "status" => {
                    item.order = 10;
                    item.keep = u8::MAX;
                    item.min_width = 5;
                }
                "vivy-plan" | "plan" => {
                    item.order = 20;
                    item.keep = u8::MAX;
                    item.min_width = 4;
                }
                "model" => {
                    item.keep = 200;
                    item.min_width = 8;
                }
                "context" => item.keep = 180,
                _ => {}
            }
            next.show(item);
        }
        if *items != next {
            *items = next;
        }
    }
}

fn input_chrome(
    mut editors: Query<(Entity, &Editor, &mut PanelCanvas), With<MainEditor>>,
    state: Res<InputContext>,
    focus: Res<KeyFocus>,
    shown: Res<ShownAgent>,
    active: Query<(), With<ActiveTurn>>,
    theme: Res<Theme>,
) {
    // Leave the question title, refusal hints and other panels' focus intact.
    if state.answering().is_some() || state.picking().is_some() || focus.0.is_some() {
        return;
    }
    let busy = shown.0.is_some_and(|agent| active.contains(agent));
    for (entity, editor, mut canvas) in &mut editors {
        let area = canvas.area();
        if state.editor() != Some(entity) || area.width < 3 || area.height < 3 {
            continue;
        }
        let background = Style::new().bg(theme.input);
        let hint_area = Rect::new(area.x, area.y, area.width, 1);
        Clear.render(hint_area, canvas.buffer_mut());
        Block::new()
            .style(background)
            .render(hint_area, canvas.buffer_mut());
        if editor.is_empty() {
            let hint = input_hint(busy, usize::from(area.width - 2));
            let inside = Rect::new(area.x + 1, area.y, area.width - 2, 1);
            Paragraph::new(Line::from(hint).dim())
                .style(background)
                .render(inside, canvas.buffer_mut());
            let placeholder = if busy {
                "Steer the current task"
            } else {
                "Ask vivy-test to do anything"
            };
            let inside = Rect::new(area.x + 3, area.y + 1, area.width.saturating_sub(4), 1);
            Paragraph::new(Line::from(placeholder).dim())
                .style(background)
                .render(inside, canvas.buffer_mut());
        }
    }
}

/// Fit complete hints rather than cutting a shortcut midway. During work,
/// stopping the turn stays discoverable even in a narrow terminal.
fn input_hint(busy: bool, width: usize) -> String {
    let items: &[&str] = if busy {
        &["Esc stop", "Enter steer", "Tab queue"]
    } else {
        &["Enter send", "Shift+Enter newline", "/ commands", "@ files"]
    };
    let mut shown = String::new();
    for item in items {
        let next = if shown.is_empty() {
            (*item).to_owned()
        } else {
            format!("{shown} · {item}")
        };
        if Line::from(next.as_str()).width() > width {
            break;
        }
        shown = next;
    }
    shown
}

/// Replace only the known upstream application-title sequence. Never rewrite
/// transcript text, arbitrary OSC payloads, prompt marks or clipboard contents.
fn brand_title(mut writes: MessageMutator<TerminalWrite>) {
    const PREFIX: &[u8] = b"\x1b]2;rig \xc2\xb7 ";
    for write in writes.read() {
        let start = if write.0.starts_with(b"\x1b[22;0t") {
            7
        } else {
            0
        };
        if write
            .0
            .get(start..)
            .is_some_and(|tail| tail.starts_with(PREFIX))
        {
            write
                .0
                .splice(start + 4..start + 7, b"vivy-test".iter().copied());
        }
    }
}

#[cfg(test)]
mod tests;
