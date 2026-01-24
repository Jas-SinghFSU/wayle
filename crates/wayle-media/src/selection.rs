use std::sync::Arc;

use wayle_common::glob::matches;

use crate::{core::player::Player, types::PlaybackState};

/// Context for player selection decisions.
pub struct SelectionContext<'a> {
    /// Available players to select from.
    pub players: &'a [Arc<Player>],
    /// Glob patterns for priority matching, checked in order.
    pub priority_patterns: &'a [String],
}

/// Selects the best player based on priority patterns, playback state, or list order.
///
/// Selection precedence:
/// 1. First player matching a priority pattern (patterns checked in order)
/// 2. First player currently playing
/// 3. First player in the list
pub fn select_best_player(ctx: &SelectionContext<'_>) -> Option<Arc<Player>> {
    select_by_priority(ctx.players, ctx.priority_patterns)
        .or_else(|| select_playing(ctx.players))
        .or_else(|| ctx.players.first().cloned())
}

fn select_by_priority(players: &[Arc<Player>], patterns: &[String]) -> Option<Arc<Player>> {
    patterns.iter().find_map(|pattern| {
        players
            .iter()
            .find(|p| matches(pattern, p.id.bus_name()))
            .cloned()
    })
}

fn select_playing(players: &[Arc<Player>]) -> Option<Arc<Player>> {
    players
        .iter()
        .find(|p| p.playback_state.get() == PlaybackState::Playing)
        .cloned()
}
