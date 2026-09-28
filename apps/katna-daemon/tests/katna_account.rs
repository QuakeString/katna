// SPDX-License-Identifier: GPL-3.0-or-later

//! The daemon's Katna account against a running Katna Server. Start one
//! (`server/katna-server/README.md`) and run with
//! `KATNA_SERVER_URL=http://127.0.0.1:8080` and `--include-ignored`. The
//! server must log codes (`KATNA_SERVER_DEV_MAILER=log` and no SMTP
//! relay); this test reads them from `KATNA_SERVER_LOG`, the server's log
//! file.

use katna_daemon::daemon::CommandError;
use katna_daemon::katna_account::Session;
use katna_daemon::secrets::Secrets;
use katna_dbus::katna_error;

/// The last code the server logged for `email` and `purpose` (`verify` or
/// `reset`). Reset codes are mailed after the server answers, so this
/// waits a little for the line.
fn logged_code(email: &str, purpose: &str) -> String {
    let path = std::env::var("KATNA_SERVER_LOG").expect("KATNA_SERVER_LOG");
    for _ in 0..50 {
        let log = std::fs::read_to_string(&path).unwrap();
        let code = log
            .lines()
            .rev()
            .filter(|line| line.contains(email) && line.contains(purpose))
            .find_map(|line| {
                let at = line.find("code=")? + 5;
                let digits: String = line[at..]
                    .chars()
                    .skip_while(|c| !c.is_ascii_digit())
                    .take(6)
                    .collect();
                (digits.len() == 6).then_some(digits)
            });
        if let Some(code) = code {
            return code;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    panic!("no {purpose} code logged for {email}");
}

fn code_of(result: Result<impl std::fmt::Debug, CommandError>) -> String {
    match result {
        Err(
            CommandError::AuthFailed(code)
            | CommandError::InvalidArgs(code)
            | CommandError::Failed(code),
        ) => code,
        other => panic!("expected an error, got {other:?}"),
    }
}

#[test]
#[ignore = "needs a Katna Server in KATNA_SERVER_URL"]
fn sign_up_confirm_and_devices() {
    smol::block_on(async {
        let laptop_secrets = Secrets::memory();
        let laptop = Session::new(&laptop_secrets).unwrap();
        assert!(!laptop.account().await.unwrap().signed_in);
        let email = format!("daemon-{}@example.com", std::process::id());

        assert_eq!(
            code_of(laptop.sign_up("nope", "correct horse").await),
            katna_error::BAD_EMAIL
        );
        assert_eq!(
            code_of(laptop.sign_up(&email, "short").await),
            katna_error::SHORT_PASSWORD
        );
        let account = laptop.sign_up(&email, "correct horse").await.unwrap();
        assert!(account.signed_in && !account.verified);
        assert!(laptop.token().await.unwrap().is_some());

        assert_eq!(
            code_of(laptop.verify("000000x").await),
            katna_error::WRONG_CODE
        );
        let account = laptop.verify(&logged_code(&email, "verify")).await.unwrap();
        assert!(account.verified);

        // A second computer signs in; the first signs it out.
        let desktop_secrets = Secrets::memory();
        let desktop = Session::new(&desktop_secrets).unwrap();
        assert_eq!(
            code_of(desktop.sign_in(&email, "wrong horse").await),
            katna_error::WRONG_PASSWORD
        );
        assert!(
            desktop
                .sign_in(&email, "correct horse")
                .await
                .unwrap()
                .verified
        );
        let devices = laptop.devices().await.unwrap();
        assert_eq!(devices.len(), 2);
        let other = devices.iter().find(|device| !device.this).unwrap();
        laptop.sign_out_device(&other.id).await.unwrap();
        // The desktop notices on its next look.
        assert!(!desktop.account().await.unwrap().signed_in);
        assert_eq!(code_of(desktop.devices().await), katna_error::SIGN_IN);

        // Reset the password from the desktop.
        desktop.reset_password(&email).await.unwrap();
        let reset = logged_code(&email, "reset");
        let account = desktop
            .confirm_reset(&email, &reset, "battery staple")
            .await
            .unwrap();
        assert!(account.signed_in);
        assert!(!laptop.account().await.unwrap().signed_in);

        // Delete it.
        assert_eq!(
            code_of(desktop.delete_account("correct horse").await),
            katna_error::WRONG_PASSWORD
        );
        desktop.delete_account("battery staple").await.unwrap();
        assert!(!desktop.account().await.unwrap().signed_in);
        // It can sign up again (it registers anew).
        assert!(
            desktop
                .sign_up(&email, "correct horse")
                .await
                .unwrap()
                .signed_in
        );
    });
}
