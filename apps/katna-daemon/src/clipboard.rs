// SPDX-License-Identifier: GPL-3.0-or-later

//! Puts text on the clipboard. The daemon has no window to own the
//! clipboard with, so it asks Plasma's clipboard (Klipper), else
//! `wl-copy` or `xclip`; on Windows, `clip`.

use std::io::Write;
use std::process::{Command, Stdio};

/// Puts `text` on the clipboard. Whether it worked.
pub(crate) async fn copy(connection: &zbus::Connection, text: &str) -> bool {
    #[cfg(unix)]
    {
        let klipper = connection
            .call_method(
                Some("org.kde.klipper"),
                "/klipper",
                Some("org.kde.klipper.klipper"),
                "setClipboardContents",
                &(text,),
            )
            .await;
        match klipper {
            Ok(_) => return true,
            Err(err) => tracing::debug!(%err, "no Klipper; trying wl-copy and xclip"),
        }
        let wayland = std::env::var_os("WAYLAND_DISPLAY").is_some();
        let tools: &[(&str, &[&str])] = if wayland {
            &[("wl-copy", &[]), ("xclip", &["-selection", "clipboard"])]
        } else {
            &[("xclip", &["-selection", "clipboard"]), ("wl-copy", &[])]
        };
        for (tool, args) in tools {
            if pipe(tool, args, text.as_bytes()).await {
                return true;
            }
        }
        tracing::warn!("could not copy: no Klipper, wl-copy or xclip");
        false
    }
    #[cfg(windows)]
    {
        let _ = connection;
        // `clip` reads UTF-16 when the text starts with its byte order mark.
        let mut utf16 = vec![0xFF, 0xFE];
        utf16.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
        pipe("clip", &[], &utf16).await
    }
}

/// Runs `tool` with `args`, writing `input` to it. Whether it worked.
async fn pipe(tool: &str, args: &[&str], input: &[u8]) -> bool {
    let mut command = Command::new(tool);
    command
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // No console window flashing up.
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let input = input.to_vec();
    let tool = tool.to_owned();
    smol::unblock(move || {
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(err) => {
                tracing::debug!(%err, tool, "cannot copy with it");
                return false;
            }
        };
        let written = child
            .stdin
            .take()
            .is_some_and(|mut stdin| stdin.write_all(&input).is_ok());
        // `wl-copy` and `xclip` stay on in the background to serve the
        // clipboard; the one started here returns once they are set up.
        child.wait().is_ok_and(|status| status.success()) && written
    })
    .await
}
