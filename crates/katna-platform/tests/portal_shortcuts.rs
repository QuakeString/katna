// SPDX-License-Identifier: GPL-3.0-or-later

//! Global shortcuts through a fake GlobalShortcuts portal on a private
//! session bus. Needs `dbus-daemon`, as `katna-daemon`'s bus tests do.
#![cfg(unix)]

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures_lite::future;
use katna_platform::shortcuts::{Keys, Shortcut, portal};
use zbus::message::Header;
use zbus::object_server::SignalEmitter;
use zbus::zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Value};
use zbus::{Connection, interface};

const PATH: &str = "/org/freedesktop/portal/desktop";
const SESSION: &str = "/org/freedesktop/portal/desktop/session/katna/s1";

/// A private `dbus-daemon`, killed on drop.
struct Bus(Child, String);

impl Bus {
    fn start() -> Self {
        let mut child = Command::new("dbus-daemon")
            .args(["--session", "--nofork", "--print-address=1"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("dbus-daemon is installed");
        let mut address = String::new();
        BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut address)
            .unwrap();
        Self(child, address.trim().to_owned())
    }

    async fn connect(&self) -> Connection {
        zbus::connection::Builder::address(self.1.as_str())
            .unwrap()
            .build()
            .await
            .unwrap()
    }
}

impl Drop for Bus {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// What the fake portal was told.
#[derive(Default)]
struct Told {
    app_id: Option<String>,
    bound: Vec<(String, String, String)>,
}

struct Registry(Arc<Mutex<Told>>);

#[interface(name = "org.freedesktop.host.portal.Registry")]
impl Registry {
    fn register(&self, app_id: String, _options: HashMap<String, OwnedValue>) {
        self.0.lock().unwrap().app_id = Some(app_id);
    }
}

struct Shortcuts {
    told: Arc<Mutex<Told>>,
    /// Whether the user says no to binding.
    refuse: bool,
}

/// Answers the request for `options`' handle token of the caller in
/// `header`, as the portal does once the user has answered.
async fn respond(
    connection: &Connection,
    header: &Header<'_>,
    options: &HashMap<String, OwnedValue>,
    response: u32,
    results: HashMap<&str, Value<'_>>,
) -> OwnedObjectPath {
    let token = options
        .get("handle_token")
        .and_then(|value| String::try_from(value.try_clone().unwrap()).ok())
        .unwrap();
    let sender = header
        .sender()
        .unwrap()
        .trim_start_matches(':')
        .replace('.', "_");
    let path = OwnedObjectPath::from(
        ObjectPath::try_from(format!("{PATH}/request/{sender}/{token}")).unwrap(),
    );
    connection
        .emit_signal(
            Option::<&str>::None,
            &path,
            "org.freedesktop.portal.Request",
            "Response",
            &(response, results),
        )
        .await
        .unwrap();
    path
}

#[interface(name = "org.freedesktop.portal.GlobalShortcuts")]
impl Shortcuts {
    async fn create_session(
        &self,
        options: HashMap<String, OwnedValue>,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] connection: &Connection,
    ) -> OwnedObjectPath {
        let results = HashMap::from([("session_handle", Value::from(SESSION))]);
        respond(connection, &header, &options, 0, results).await
    }

    async fn bind_shortcuts(
        &self,
        session: OwnedObjectPath,
        shortcuts: Vec<(String, HashMap<String, OwnedValue>)>,
        _parent: String,
        options: HashMap<String, OwnedValue>,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] connection: &Connection,
    ) -> OwnedObjectPath {
        assert_eq!(session.as_str(), SESSION);
        let text = |map: &HashMap<String, OwnedValue>, key: &str| {
            String::try_from(map[key].try_clone().unwrap()).unwrap()
        };
        self.told.lock().unwrap().bound = shortcuts
            .iter()
            .map(|(id, map)| {
                (
                    id.clone(),
                    text(map, "description"),
                    text(map, "preferred_trigger"),
                )
            })
            .collect();
        let response = if self.refuse { 1 } else { 0 };
        respond(connection, &header, &options, response, HashMap::new()).await
    }

    // The portal's property is in lower case, unlike zbus' default.
    #[zbus(property, name = "version")]
    fn version(&self) -> u32 {
        1
    }

    #[zbus(signal)]
    async fn activated(
        emitter: &SignalEmitter<'_>,
        session: ObjectPath<'_>,
        id: &str,
        timestamp: u64,
        options: HashMap<&str, Value<'_>>,
    ) -> zbus::Result<()>;
}

async fn fake_portal(bus: &Bus, refuse: bool) -> (Connection, Arc<Mutex<Told>>) {
    let told = Arc::new(Mutex::new(Told::default()));
    let connection = zbus::connection::Builder::address(bus.1.as_str())
        .unwrap()
        .name("org.freedesktop.portal.Desktop")
        .unwrap()
        .serve_at(PATH, Registry(told.clone()))
        .unwrap()
        .serve_at(
            PATH,
            Shortcuts {
                told: told.clone(),
                refuse,
            },
        )
        .unwrap()
        .build()
        .await
        .unwrap();
    (connection, told)
}

fn quick_capture() -> Vec<Shortcut> {
    vec![
        Shortcut {
            action: "capture-task".to_owned(),
            name: "New task".to_owned(),
            keys: Keys::meta_alt('t'),
        },
        Shortcut {
            action: "capture-note".to_owned(),
            name: "New note".to_owned(),
            keys: Keys::meta_alt('n'),
        },
    ]
}

async fn within<T>(seconds: u64, what: impl Future<Output = T>) -> T {
    future::or(what, async {
        async_io_sleep(seconds).await;
        panic!("nothing within {seconds} s");
    })
    .await
}

/// A timer without an extra crate: a thread that sleeps.
async fn async_io_sleep(seconds: u64) {
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(seconds));
        let _ = tx.send(());
    });
    while rx.try_recv().is_err() {
        future::yield_now().await;
    }
}

#[test]
fn binds_through_the_portal_and_hears_presses() {
    let bus = Bus::start();
    future::block_on(async {
        let (portal_bus, told) = fake_portal(&bus, false).await;
        let app = bus.connect().await;
        let pressed = Arc::new(Mutex::new(Vec::<String>::new()));
        let heard = pressed.clone();
        let handler = Arc::new(move |action: &str| heard.lock().unwrap().push(action.to_owned()));
        let serving = portal::serve(&app, "in.invenia.katna.Mail", quick_capture(), handler);
        let pressing = async {
            // Bound once the portal has the shortcuts.
            while told.lock().unwrap().bound.is_empty() {
                future::yield_now().await;
            }
            let emitter = SignalEmitter::new(&portal_bus, PATH).unwrap();
            loop {
                Shortcuts::activated(
                    &emitter,
                    ObjectPath::try_from(SESSION).unwrap(),
                    "capture-note",
                    1,
                    HashMap::new(),
                )
                .await
                .unwrap();
                if !pressed.lock().unwrap().is_empty() {
                    break;
                }
                async_io_sleep(0).await;
            }
        };
        within(10, future::or(async { serving.await.unwrap() }, pressing)).await;
        let told = told.lock().unwrap();
        assert_eq!(told.app_id.as_deref(), Some("in.invenia.katna.Mail"));
        assert_eq!(
            told.bound,
            [
                (
                    "capture-task".into(),
                    "New task".into(),
                    "ALT+LOGO+t".into()
                ),
                (
                    "capture-note".into(),
                    "New note".into(),
                    "ALT+LOGO+n".into()
                ),
            ]
        );
        assert_eq!(pressed.lock().unwrap()[0], "capture-note");
    });
}

#[test]
fn a_refusal_or_no_portal_ends_quietly() {
    let bus = Bus::start();
    future::block_on(async {
        let app = bus.connect().await;
        let handler = Arc::new(|_: &str| panic!("no shortcut is pressed"));
        // No portal on the bus at all.
        within(
            10,
            portal::serve(&app, "app", quick_capture(), handler.clone()),
        )
        .await
        .unwrap();
        let (_portal, told) = fake_portal(&bus, true).await;
        let app = bus.connect().await;
        within(10, portal::serve(&app, "app", quick_capture(), handler))
            .await
            .unwrap();
        assert_eq!(told.lock().unwrap().bound.len(), 2);
    });
}
