// SPDX-License-Identifier: GPL-3.0-only

//! Push this session's settings to a shared compositor.
//!
//! Under the persistent-compositor model the compositor runs as its own system user and outlives
//! every session, so it reads its own configuration and never sees this user's. Settings changed
//! here -- night shift, shortcuts, tiling -- would simply never take effect.
//!
//! This forwards them over the wayland connection the daemon already holds. The compositor
//! advertises `agentos_session_config_v1` only to the user whose session holds the screen, so on
//! a classic per-session compositor (or for any other user) the global is absent and this is inert.
//!
//! Values are forwarded verbatim -- the same serialized text cosmic-config writes -- so there is
//! no second representation to keep in sync.

use std::path::PathBuf;

mod generated {
    use cctk::wayland_client;
    // wayland-scanner needs the interface tables in scope under this exact name.
    pub mod __interfaces {
        wayland_scanner::generate_interfaces!("resources/protocols/agentos-session-config.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_client_code!("resources/protocols/agentos-session-config.xml");
}

pub use generated::agentos_session_config_v1::AgentosSessionConfigV1;

/// Domains the compositor accepts. Kept in step with its allowlist; it drops anything else, so
/// sending more would only be noise.
pub const DOMAINS: &[&str] = &[
    "com.system76.CosmicComp",
    "com.system76.CosmicTk",
    "com.system76.CosmicSettings.Shortcuts",
    "com.system76.CosmicSettings.WindowRules",
    "inc.kora.VoiceMode",
    // Until the compositor takes the new name; the user dir is moved on its first start.
    "com.playtron.VoiceMode",
];

const CONFIG_VERSION: u64 = 1;

/// Send every key this user currently has set, so the compositor starts from their values rather
/// than whatever the previous session left in its store.
pub fn push_snapshot(sink: &AgentosSessionConfigV1) {
    let mut sent = 0usize;
    for domain in DOMAINS {
        for (key, value) in read_domain(domain) {
            sink.set_entry(domain.to_string(), key, value);
            sent += 1;
        }
    }
    log::info!("session-config: sent {sent} entries to the compositor");
}

/// Re-read one key and send it. Re-reading rather than trusting the event keeps coalesced writes
/// correct: the file is the value the compositor should end up with.
pub fn push_key(sink: &AgentosSessionConfigV1, domain: &str, key: &str) {
    if let Some(value) = read_key(domain, key) {
        sink.set_entry(domain.to_string(), key.to_string(), value);
        log::debug!("session-config: forwarded {domain}/{key}");
    }
}

/// Watch every domain, reporting `(domain, key)` for each change. The returned watchers must be
/// held: dropping them stops the notifications.
pub fn watch_domains(
    tx: calloop::channel::Sender<(&'static str, String)>,
) -> Vec<notify::RecommendedWatcher> {
    DOMAINS
        .iter()
        .filter_map(|domain| {
            let config = cosmic_config::Config::new(domain, CONFIG_VERSION).ok()?;
            let tx = tx.clone();
            config
                .watch(move |_, keys| {
                    for key in keys {
                        let _ = tx.send((domain, key.clone()));
                    }
                })
                .ok()
        })
        .collect()
}

/// Every key currently set for a domain, as the raw serialized text on disk.
fn read_domain(domain: &str) -> Vec<(String, String)> {
    let Some(dir) = domain_dir(domain) else {
        return Vec::new();
    };
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter(|e| e.file_type().is_ok_and(|t| t.is_file()))
        .filter_map(|e| {
            let key = e.file_name().into_string().ok()?;
            // Skip cosmic-config's in-flight temp files; their rename notifies us anyway.
            if key.starts_with('.') {
                return None;
            }
            Some((key, std::fs::read_to_string(e.path()).ok()?))
        })
        .collect()
}

fn read_key(domain: &str, key: &str) -> Option<String> {
    std::fs::read_to_string(domain_dir(domain)?.join(key)).ok()
}

fn domain_dir(domain: &str) -> Option<PathBuf> {
    Some(
        dirs::config_dir()?
            .join("cosmic")
            .join(domain)
            .join(format!("v{CONFIG_VERSION}")),
    )
}
