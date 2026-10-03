// SPDX-License-Identifier: GPL-3.0-or-later

//! Which rules run on an account's mail service instead of in Katna
//! (`docs/ARCHITECTURE.md` §9.4): as Gmail filters ([`crate::gmail_filters`])
//! or in the account's Sieve script ([`crate::sieve`]).
//!
//! Rules run in list order, and "stop" ends the list for the mail. The
//! service runs its rules when mail arrives, before Katna sees it, so an
//! account's rules on the service are always the first of its list: from
//! the first rule that has to stay in Katna on, every later rule of the
//! account stays too ([`RunsNote::Order`]).

use katna_core::AccountId;
use katna_store::rules::{Rule, RunsNote, RunsOn};

/// What [`plan`] decided for one rule of an account: what to send to the
/// service, or why it stays in Katna.
pub type Verdict<T> = Result<T, RunsNote>;

/// The rules of `rules` that look at mail of `account` (switched on,
/// covering it), in list order, each with its verdict on `service`.
/// `translate` gets a rule and whether a later rule of the account
/// follows; it says what to send, or why the service can't run it.
pub fn plan<T>(
    rules: &[Rule],
    account: AccountId,
    service: RunsOn,
    mut translate: impl FnMut(&Rule, bool) -> Verdict<T>,
) -> Vec<(i64, Verdict<T>)> {
    let mine: Vec<&Rule> = rules
        .iter()
        .filter(|r| r.enabled && r.covers(account))
        .collect();
    let mut out = Vec::with_capacity(mine.len());
    let mut in_katna = false;
    for (ix, rule) in mine.iter().enumerate() {
        let verdict = if in_katna {
            Err(RunsNote::Order { service })
        } else {
            translate(rule, ix + 1 < mine.len())
        };
        in_katna |= verdict.is_err();
        out.push((rule.id, verdict));
    }
    out
}

/// Where a rule runs, from where it runs on each of its accounts
/// (`None`: that account's service runs no rules), and the note the
/// rule shows: on the service when every account's service runs it,
/// else in Katna, with the first reason one of them gave.
pub fn runs(per_account: &[Result<RunsOn, Option<RunsNote>>]) -> (RunsOn, Option<RunsNote>) {
    let first = per_account.first().and_then(|r| r.as_ref().ok()).copied();
    if let Some(service) = first
        && per_account
            .iter()
            .all(|r| r.as_ref().ok() == Some(&service))
    {
        return (service, None);
    }
    let note = per_account
        .iter()
        .find_map(|r| r.as_ref().err().cloned().flatten());
    (RunsOn::Katna, note)
}

#[cfg(test)]
mod tests {
    use katna_store::rules::{Action, Comparator, Condition, Field};

    use super::*;

    fn rule(id: i64, accounts: &[i64], actions: Vec<Action>) -> Rule {
        Rule {
            id,
            name: format!("Rule {id}"),
            conditions: vec![Condition {
                field: Field::Subject,
                comparator: Comparator::Contains,
                value: "x".into(),
            }],
            actions,
            accounts: accounts.to_vec(),
            ..Rule::default()
        }
    }

    #[test]
    fn rules_after_one_that_stays_in_katna_stay_too() {
        let mut off = rule(2, &[1], vec![Action::Star]);
        off.enabled = false;
        let rules = [
            rule(1, &[1], vec![Action::MarkRead]),
            off,
            rule(3, &[2], vec![Action::DontNotify]),
            rule(4, &[1, 2], vec![Action::DontNotify]),
            rule(5, &[1], vec![Action::MarkRead]),
        ];
        let mut later = Vec::new();
        let plan = plan(&rules, AccountId(1), RunsOn::Sieve, |r, more| {
            later.push((r.id, more));
            match r.actions[0] {
                Action::DontNotify => Err(RunsNote::Action {
                    service: RunsOn::Sieve,
                    action: Action::DontNotify,
                }),
                _ => Ok(r.id * 10),
            }
        });
        assert_eq!(
            plan,
            [
                (1, Ok(10)),
                (
                    4,
                    Err(RunsNote::Action {
                        service: RunsOn::Sieve,
                        action: Action::DontNotify
                    })
                ),
                (
                    5,
                    Err(RunsNote::Order {
                        service: RunsOn::Sieve
                    })
                ),
            ]
        );
        // Switched off, other accounts' and later rules aren't asked.
        assert_eq!(later, [(1, true), (4, true)]);
    }

    #[test]
    fn a_rule_runs_on_the_service_only_when_all_its_accounts_do() {
        let note = RunsNote::SignIn;
        assert_eq!(runs(&[Ok(RunsOn::Gmail)]), (RunsOn::Gmail, None));
        assert_eq!(
            runs(&[Ok(RunsOn::Sieve), Ok(RunsOn::Sieve)]),
            (RunsOn::Sieve, None)
        );
        assert_eq!(
            runs(&[Ok(RunsOn::Gmail), Ok(RunsOn::Sieve)]),
            (RunsOn::Katna, None)
        );
        assert_eq!(runs(&[Ok(RunsOn::Gmail), Err(None)]), (RunsOn::Katna, None));
        assert_eq!(
            runs(&[Ok(RunsOn::Gmail), Err(Some(note.clone()))]),
            (RunsOn::Katna, Some(note))
        );
        assert_eq!(runs(&[]), (RunsOn::Katna, None));
    }
}
