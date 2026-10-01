// SPDX-License-Identifier: GPL-3.0-or-later
// Katna Server's admin page. Every call carries X-Katna-Admin; the session
// is an HttpOnly cookie this script never sees. Text from the server goes
// in with textContent only.
"use strict";

const $ = (id) => document.getElementById(id);
const MONTHS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
const LONG_MONTHS = ["January", "February", "March", "April", "May", "June", "July", "August",
  "September", "October", "November", "December"];

let page = null;     // the state the server last sent
let step = "password"; // see STEPS

async function call(method, path, body) {
  const response = await fetch(path, {
    method,
    credentials: "same-origin",
    headers: Object.assign({ "X-Katna-Admin": "1" }, body ? { "Content-Type": "application/json" } : {}),
    body: body ? JSON.stringify(body) : undefined,
  });
  let data = null;
  try { data = await response.json(); } catch (_) { /* 202 and 204 have no body */ }
  return { status: response.status, data };
}

function dollars(micros) {
  return (micros / 1e6).toFixed(2);
}

function micros(text) {
  const value = Number(String(text).trim().replace(/^\$/, ""));
  return Number.isFinite(value) && value >= 0 ? Math.round(value * 1e6) : null;
}

function whole(text) {
  const value = Number(String(text).trim());
  return Number.isInteger(value) && value >= 0 ? value : null;
}

function monthName(yyyymm, long) {
  const index = (yyyymm % 100) - 1;
  return long ? `${LONG_MONTHS[index]} ${Math.floor(yyyymm / 100)}` : MONTHS[index];
}

// Signing in. Steps: "password" then "code"; or, for the first password,
// "setup" (mail a code) then "choose" (the code and the new password).

const STEPS = {
  password: {
    intro: "For whoever runs this server, not Katna accounts. Sign in with the admin address and password.",
    fields: ["password-field"], button: "Continue",
    other: "First time here? Choose the password", otherStep: "setup",
    foot: "Signed in for 12 hours on this browser.",
  },
  code: {
    intro: "We mailed a code to the admin address.",
    fields: ["code-field"], button: "Sign in",
    other: "Start again", otherStep: "password",
    foot: "The code works for 30 minutes.",
  },
  setup: {
    intro: "Choose the admin password. Type the admin address from the server's .env (KATNA_SERVER_ADMIN_EMAILS) and we'll mail it a code, so only its owner can set the password.",
    fields: [], button: "Mail me a code",
    other: "Already have a password? Sign in", otherStep: "password",
    foot: "Forgot the password? On the server: katna-server admin-password",
  },
  choose: {
    intro: "Type the code we mailed to the admin address, then the new password twice.",
    fields: ["code-field", "new-password-field", "again-field"], button: "Save and sign in",
    other: "Start again", otherStep: "setup",
    foot: "At least 8 characters.",
  },
};
const STEP_FIELDS = ["password-field", "code-field", "new-password-field", "again-field"];

function showStep(name) {
  step = name;
  const s = STEPS[name];
  $("signin-intro").textContent = s.intro;
  for (const id of STEP_FIELDS) $(id).hidden = !s.fields.includes(id);
  $("signin-button").textContent = s.button;
  $("switch-step").textContent = s.other;
  $("signin-foot").textContent = s.foot;
  for (const id of ["password", "code", "new-password", "again"]) $(id).value = "";
  const first = s.fields.length && name !== "password" ? s.fields[0].replace("-field", "") : "email";
  (name === "password" && $("email").value ? $("password") : $(first)).focus();
}

async function showSignIn() {
  $("admin").hidden = true;
  $("signin").hidden = false;
  let first = "password";
  try {
    const { status, data } = await call("GET", "/admin/api/status");
    if (status === 200 && data.setup) first = "setup";
  } catch (_) { /* the sign-in step then */ }
  showStep(first);
}

function signInProblem(text) {
  const problem = $("signin-problem");
  problem.textContent = text || "";
  problem.hidden = !text;
}

$("switch-step").addEventListener("click", () => {
  signInProblem("");
  showStep(STEPS[step].otherStep);
});

function codeProblem(status, data) {
  if (data && data.code === "code_expired") return "That code has run out. Start again for a new one.";
  if (status === 429) return "Too many wrong codes. Try again tomorrow.";
  return "Wrong code.";
}

$("signin-form").addEventListener("submit", async (event) => {
  event.preventDefault();
  signInProblem("");
  const button = $("signin-button");
  button.disabled = true;
  try {
    if (step === "password") {
      const { status } = await call("POST", "/admin/api/sign-in", {
        email: $("email").value, password: $("password").value,
      });
      if (status === 202) showStep("code");
      else if (status === 429) signInProblem("Too many tries. Wait a few minutes.");
      else signInProblem("That address and password can't open this page.");
    } else if (step === "code") {
      const { status, data } = await call("POST", "/admin/api/code", {
        email: $("email").value, code: $("code").value,
      });
      if (status === 204) await load();
      else signInProblem(codeProblem(status, data));
    } else if (step === "setup") {
      const { status } = await call("POST", "/admin/api/setup", { email: $("email").value });
      if (status === 202) showStep("choose");
      else if (status === 429) signInProblem("Too many codes mailed. Wait an hour.");
      else signInProblem("The code could not be mailed. Check the server's mail settings.");
    } else {
      if ($("new-password").value !== $("again").value) {
        signInProblem("The two passwords differ.");
        return;
      }
      if ([...$("new-password").value].length < 8) {
        signInProblem("The password needs at least 8 characters.");
        return;
      }
      const { status, data } = await call("POST", "/admin/api/setup/finish", {
        email: $("email").value, code: $("code").value, password: $("new-password").value,
      });
      if (status === 204) await load();
      else signInProblem(codeProblem(status, data));
    }
  } catch (_) {
    signInProblem("The server can't be reached.");
  } finally {
    button.disabled = false;
  }
});

$("sign-out").addEventListener("click", async () => {
  await call("POST", "/admin/api/sign-out");
  page = null;
  showSignIn();
});

// The page.

async function load() {
  const { status, data } = await call("GET", "/admin/api/state");
  if (status === 401) {
    showSignIn();
    return;
  }
  if (status !== 200) {
    signInProblem("The server can't show the page just now.");
    showSignIn();
    return;
  }
  page = data;
  $("signin").hidden = true;
  $("admin").hidden = false;
  render();
}

function option(select, value, text) {
  const item = document.createElement("option");
  item.value = value;
  item.textContent = text;
  select.append(item);
}

function fillServices(select, chosen, allowNone) {
  select.replaceChildren();
  if (allowNone) option(select, "", "None");
  for (const service of page.services) {
    if (service.has_key) option(select, service.id, service.name);
  }
  select.value = chosen ? chosen.provider : "";
}

function render() {
  const s = page.settings;
  const stats = page.stats;
  $("who").textContent = page.email;
  $("month-name").textContent = monthName(page.month, true);

  const main = page.services.find((service) => s.main && service.id === s.main.provider);
  const live = s.on && main;
  $("status").classList.toggle("off", !live);
  $("status-text").textContent = live ? `On: answering through ${main.name}` : "Off";

  $("spent").textContent = `$${dollars(stats.cost_micros)}`;
  $("budget-of").textContent = `of $${dollars(s.budget_micros)} budget`;
  const used = s.budget_micros > 0 ? Math.min(100, (100 * stats.cost_micros) / s.budget_micros) : 100;
  $("meter-fill").style.width = `${used}%`;
  $("meter").setAttribute("aria-valuenow", String(Math.round(used)));
  $("requests").textContent = stats.requests.toLocaleString("en");
  $("trial").textContent = stats.trial_accounts.toLocaleString("en");
  $("paid").textContent = stats.paid_accounts.toLocaleString("en");
  $("capped").textContent = stats.capped_accounts.toLocaleString("en");

  fillServices($("main-provider"), s.main, false);
  $("main-model").value = s.main ? s.main.model : "";
  fillServices($("fallback-provider"), s.fallback, true);
  $("fallback-model").value = s.fallback ? s.fallback.model : "";
  $("fallback-model").disabled = !s.fallback;
  $("test").textContent = s.fallback ? "Test both" : "Test";

  const keys = $("keys");
  keys.replaceChildren();
  for (const service of page.services) {
    const chip = document.createElement("span");
    chip.className = service.has_key ? "chip has-key" : "chip";
    chip.textContent = service.has_key ? `${service.name} ✓` : `${service.name}: no key`;
    keys.append(chip);
  }

  $("budget").value = dollars(s.budget_micros);
  $("cap").value = dollars(s.account_cap_micros);
  $("trial-days").value = String(s.trial_days);
  $("per-hour").value = String(s.per_hour);
  $("price-in").value = dollars(s.price_in_micros);
  $("price-out").value = dollars(s.price_out_micros);
  $("on").checked = s.on;

  const months = $("months");
  months.replaceChildren();
  const top = Math.max(1, ...stats.months.map(([, cost]) => cost));
  stats.months.forEach(([month, cost], index) => {
    const now = index === stats.months.length - 1;
    const column = document.createElement("div");
    if (now) column.className = "now";
    const bar = document.createElement("div");
    bar.className = "col";
    bar.style.height = `${Math.max(4, Math.round((48 * cost) / top))}px`;
    const label = document.createElement("div");
    label.className = "label";
    label.textContent = now ? `${monthName(month)} $${dollars(cost)}` : monthName(month);
    column.title = `${monthName(month, true)}: $${dollars(cost)}`;
    column.append(bar, label);
    months.append(column);
  });
  changed();
}

// Editing.

function choice(providerId, modelId) {
  const provider = $(providerId).value;
  return provider ? { provider, model: $(modelId).value.trim() } : null;
}

/// The settings as the page shows them, or null with the bad field marked.
function edited() {
  const fields = [
    ["budget", micros], ["cap", micros], ["price-in", micros], ["price-out", micros],
    ["trial-days", whole], ["per-hour", whole],
  ];
  const values = {};
  let ok = true;
  for (const [id, read] of fields) {
    const value = read($(id).value);
    $(id).setAttribute("aria-invalid", value === null ? "true" : "false");
    if (value === null) ok = false;
    values[id] = value;
  }
  if (!ok) return null;
  return {
    on: $("on").checked,
    main: choice("main-provider", "main-model"),
    fallback: choice("fallback-provider", "fallback-model"),
    trial_days: values["trial-days"],
    account_cap_micros: values.cap,
    budget_micros: values.budget,
    price_in_micros: values["price-in"],
    price_out_micros: values["price-out"],
    per_hour: values["per-hour"],
  };
}

function changed() {
  const now = edited();
  const differs = !now || JSON.stringify(now) !== JSON.stringify(page.settings);
  $("save").disabled = !now || !differs;
  $("undo").disabled = !differs;
}

function usualModel(providerId, modelId) {
  const service = page.services.find((s) => s.id === $(providerId).value);
  $(modelId).value = service ? service.model : "";
  $(modelId).disabled = !service;
}

$("main-provider").addEventListener("change", () => { usualModel("main-provider", "main-model"); changed(); });
$("fallback-provider").addEventListener("change", () => { usualModel("fallback-provider", "fallback-model"); changed(); });
for (const id of ["main-model", "fallback-model", "budget", "cap", "trial-days", "per-hour", "price-in", "price-out"]) {
  $(id).addEventListener("input", changed);
}
$("on").addEventListener("change", changed);

$("undo").addEventListener("click", () => {
  $("save-result").textContent = "";
  render();
});

$("save").addEventListener("click", async () => {
  const settings = edited();
  if (!settings) return;
  const result = $("save-result");
  $("save").disabled = true;
  const { status, data } = await call("POST", "/admin/api/settings", settings);
  if (status === 200) {
    page = data;
    render();
    result.className = "small good";
    result.textContent = "Saved. Katna AI uses this from now on.";
  } else if (status === 401) {
    showSignIn();
  } else {
    result.className = "small bad";
    result.textContent = data && data.error ? `Not saved: ${data.error}.` : "Not saved.";
    changed();
  }
});

$("test").addEventListener("click", async () => {
  const result = $("test-result");
  const button = $("test");
  button.disabled = true;
  result.className = "small muted";
  result.textContent = "Asking…";
  try {
    const { status, data } = await call("POST", "/admin/api/test");
    if (status === 401) { showSignIn(); return; }
    if (status !== 200 || data.length === 0) {
      result.className = "small bad";
      result.textContent = "No service chosen.";
      return;
    }
    const name = (id) => (page.services.find((s) => s.id === id) || { name: id }).name;
    const parts = data.map((t) => (t.ms !== null ? `${name(t.provider)} ${t.ms} ms` : `${name(t.provider)} ${t.problem}`));
    const allGood = data.every((t) => t.ms !== null);
    result.className = allGood ? "small good" : "small bad";
    result.textContent = (allGood ? "Answered: " : "") + parts.join(", ");
  } finally {
    button.disabled = false;
  }
});

load().catch(() => {
  signInProblem("The server can't be reached.");
  showSignIn();
});
