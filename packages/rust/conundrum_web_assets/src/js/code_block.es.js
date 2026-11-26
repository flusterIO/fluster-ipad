const applyCopyConundrumCodeBlockListeners = () => {
  const ems = document.getElementsByClassName("cdrm-codeblock");
  for (var i2 = 0; i2 < ems.length; i2++) {
    const item = ems.item(i2);
    item.addEventListener("mouseenter", (e) => {
      e.target.classList.add("hovered");
    });
    item.addEventListener("mouseleave", (e) => {
      e.target.classList.add("hovered");
    });
  }
  const icons = document.getElementsByClassName("cdrm-codeblock-icon");
  for (var i2 = 0; i2 < icons.length; i2++) {
    const item = icons.item(i2);
    item.addEventListener("click", async (e) => {
      Promise.resolve().then(() => methods_es).then((a) => a.onCopyCodeBlockClick(e));
    });
  }
};
(() => {
  applyCopyConundrumCodeBlockListeners();
  window.addEventListener("cdrm-content-loaded", applyCopyConundrumCodeBlockListeners);
})();
const l = {
  fatal: 0,
  error: 0,
  warn: 1,
  log: 2,
  info: 3,
  success: 3,
  fail: 3,
  debug: 4,
  trace: 5,
  verbose: Number.POSITIVE_INFINITY
}, d = {
  // Silent
  silent: {
    level: -1
  },
  // Level 0
  fatal: {
    level: l.fatal
  },
  error: {
    level: l.error
  },
  // Level 1
  warn: {
    level: l.warn
  },
  // Level 2
  log: {
    level: l.log
  },
  // Level 3
  info: {
    level: l.info
  },
  success: {
    level: l.success
  },
  fail: {
    level: l.fail
  },
  ready: {
    level: l.info
  },
  start: {
    level: l.info
  },
  box: {
    level: l.info
  },
  // Level 4
  debug: {
    level: l.debug
  },
  // Level 5
  trace: {
    level: l.trace
  },
  // Verbose
  verbose: {
    level: l.verbose
  }
};
function f(s) {
  if (s === null || typeof s != "object")
    return false;
  const t = Object.getPrototypeOf(s);
  return t !== null && t !== Object.prototype && Object.getPrototypeOf(t) !== null || Symbol.iterator in s ? false : Symbol.toStringTag in s ? Object.prototype.toString.call(s) === "[object Module]" : true;
}
function g$1(s, t, e = ".", r) {
  if (!f(t))
    return g$1(s, {}, e);
  const o = Object.assign({}, t);
  for (const n in s) {
    if (n === "__proto__" || n === "constructor")
      continue;
    const p2 = s[n];
    p2 != null && (Array.isArray(p2) && Array.isArray(o[n]) ? o[n] = [...p2, ...o[n]] : f(p2) && f(o[n]) ? o[n] = g$1(
      p2,
      o[n],
      (e ? `${e}.` : "") + n.toString()
    ) : o[n] = p2);
  }
  return o;
}
function m(s) {
  return (...t) => (
    // eslint-disable-next-line unicorn/no-array-reduce
    t.reduce((e, r) => g$1(e, r, ""), {})
  );
}
const w$1 = m();
function v$1(s) {
  return Object.prototype.toString.call(s) === "[object Object]";
}
function L$1(s) {
  return !(!v$1(s) || !s.message && !s.args || s.stack);
}
let h = false;
const y$1 = [];
class i {
  options;
  _lastLog;
  _mockFn;
  /**
   * Creates an instance of Consola with specified options or defaults.
   *
   * @param {Partial<ConsolaOptions>} [options={}] - Configuration options for the Consola instance.
   */
  constructor(t = {}) {
    const e = t.types || d;
    this.options = w$1(
      {
        ...t,
        defaults: { ...t.defaults },
        level: u(t.level, e),
        reporters: [...t.reporters || []]
      },
      {
        types: d,
        throttle: 1e3,
        throttleMin: 5,
        formatOptions: {
          date: true,
          colors: false,
          compact: true
        }
      }
    );
    for (const r in e) {
      const o = {
        type: r,
        ...this.options.defaults,
        ...e[r]
      };
      this[r] = this._wrapLogFn(o), this[r].raw = this._wrapLogFn(
        o,
        true
      );
    }
    this.options.mockFn && this.mockTypes(), this._lastLog = {};
  }
  /**
   * Gets the current log level of the Consola instance.
   *
   * @returns {number} The current log level.
   */
  get level() {
    return this.options.level;
  }
  /**
   * Sets the minimum log level that will be output by the instance.
   *
   * @param {number} level - The new log level to set.
   */
  set level(t) {
    this.options.level = u(
      t,
      this.options.types,
      this.options.level
    );
  }
  /**
   * Displays a prompt to the user and returns the response.
   * Throw an error if `prompt` is not supported by the current configuration.
   *
   * @template T
   * @param {string} message - The message to display in the prompt.
   * @param {T} [opts] - Optional options for the prompt. See {@link PromptOptions}.
   * @returns {promise<T>} A promise that infer with the prompt options. See {@link PromptOptions}.
   */
  prompt(t, e) {
    if (!this.options.prompt)
      throw new Error("prompt is not supported!");
    return this.options.prompt(t, e);
  }
  /**
   * Creates a new instance of Consola, inheriting options from the current instance, with possible overrides.
   *
   * @param {Partial<ConsolaOptions>} options - Optional overrides for the new instance. See {@link ConsolaOptions}.
   * @returns {ConsolaInstance} A new Consola instance. See {@link ConsolaInstance}.
   */
  create(t) {
    const e = new i({
      ...this.options,
      ...t
    });
    return this._mockFn && e.mockTypes(this._mockFn), e;
  }
  /**
   * Creates a new Consola instance with the specified default log object properties.
   *
   * @param {InputLogObject} defaults - Default properties to include in any log from the new instance. See {@link InputLogObject}.
   * @returns {ConsolaInstance} A new Consola instance. See {@link ConsolaInstance}.
   */
  withDefaults(t) {
    return this.create({
      ...this.options,
      defaults: {
        ...this.options.defaults,
        ...t
      }
    });
  }
  /**
   * Creates a new Consola instance with a specified tag, which will be included in every log.
   *
   * @param {string} tag - The tag to include in each log of the new instance.
   * @returns {ConsolaInstance} A new Consola instance. See {@link ConsolaInstance}.
   */
  withTag(t) {
    return this.withDefaults({
      tag: this.options.defaults.tag ? this.options.defaults.tag + ":" + t : t
    });
  }
  /**
   * Adds a custom reporter to the Consola instance.
   * Reporters will be called for each log message, depending on their implementation and log level.
   *
   * @param {ConsolaReporter} reporter - The reporter to add. See {@link ConsolaReporter}.
   * @returns {Consola} The current Consola instance.
   */
  addReporter(t) {
    return this.options.reporters.push(t), this;
  }
  /**
   * Removes a custom reporter from the Consola instance.
   * If no reporter is specified, all reporters will be removed.
   *
   * @param {ConsolaReporter} reporter - The reporter to remove. See {@link ConsolaReporter}.
   * @returns {Consola} The current Consola instance.
   */
  removeReporter(t) {
    if (t) {
      const e = this.options.reporters.indexOf(t);
      if (e !== -1)
        return this.options.reporters.splice(e, 1);
    } else
      this.options.reporters.splice(0);
    return this;
  }
  /**
   * Replaces all reporters of the Consola instance with the specified array of reporters.
   *
   * @param {ConsolaReporter[]} reporters - The new reporters to set. See {@link ConsolaReporter}.
   * @returns {Consola} The current Consola instance.
   */
  setReporters(t) {
    return this.options.reporters = Array.isArray(t) ? t : [t], this;
  }
  wrapAll() {
    this.wrapConsole(), this.wrapStd();
  }
  restoreAll() {
    this.restoreConsole(), this.restoreStd();
  }
  /**
   * Overrides console methods with Consola logging methods for consistent logging.
   */
  wrapConsole() {
    for (const t in this.options.types)
      console["__" + t] || (console["__" + t] = console[t]), console[t] = this[t].raw;
  }
  /**
   * Restores the original console methods, removing Consola overrides.
   */
  restoreConsole() {
    for (const t in this.options.types)
      console["__" + t] && (console[t] = console["__" + t], delete console["__" + t]);
  }
  /**
   * Overrides standard output and error streams to redirect them through Consola.
   */
  wrapStd() {
    this._wrapStream(this.options.stdout, "log"), this._wrapStream(this.options.stderr, "log");
  }
  _wrapStream(t, e) {
    t && (t.__write || (t.__write = t.write), t.write = (r) => {
      this[e].raw(String(r).trim());
    });
  }
  /**
   * Restores the original standard output and error streams, removing the Consola redirection.
   */
  restoreStd() {
    this._restoreStream(this.options.stdout), this._restoreStream(this.options.stderr);
  }
  _restoreStream(t) {
    t && t.__write && (t.write = t.__write, delete t.__write);
  }
  /**
   * Pauses logging, queues incoming logs until resumed.
   */
  pauseLogs() {
    h = true;
  }
  /**
   * Resumes logging, processing any queued logs.
   */
  resumeLogs() {
    h = false;
    const t = y$1.splice(0);
    for (const e of t)
      e[0]._logFn(e[1], e[2]);
  }
  /**
   * Replaces logging methods with mocks if a mock function is provided.
   *
   * @param {ConsolaOptions["mockFn"]} mockFn - The function to use for mocking logging methods. See {@link ConsolaOptions["mockFn"]}.
   */
  mockTypes(t) {
    const e = t || this.options.mockFn;
    if (this._mockFn = e, typeof e == "function")
      for (const r in this.options.types)
        this[r] = e(r, this.options.types[r]) || this[r], this[r].raw = this[r];
  }
  _wrapLogFn(t, e) {
    return (...r) => {
      if (h) {
        y$1.push([this, t, r, e]);
        return;
      }
      return this._logFn(t, r, e);
    };
  }
  _logFn(t, e, r) {
    if ((t.level || 0) > this.level)
      return false;
    const o = {
      date: /* @__PURE__ */ new Date(),
      args: [],
      ...t,
      level: u(t.level, this.options.types)
    };
    !r && e.length === 1 && L$1(e[0]) ? Object.assign(o, e[0]) : o.args = [...e], o.message && (o.args.unshift(o.message), delete o.message), o.additional && (Array.isArray(o.additional) || (o.additional = o.additional.split(`
`)), o.args.push(`
` + o.additional.join(`
`)), delete o.additional), o.type = typeof o.type == "string" ? o.type.toLowerCase() : "log", o.tag = typeof o.tag == "string" ? o.tag : "";
    const n = (a = false) => {
      const c = (this._lastLog.count || 0) - this.options.throttleMin;
      if (this._lastLog.object && c > 0) {
        const _ = [...this._lastLog.object.args];
        c > 1 && _.push(`(repeated ${c} times)`), this._log({ ...this._lastLog.object, args: _ }), this._lastLog.count = 1;
      }
      a && (this._lastLog.object = o, this._log(o));
    };
    clearTimeout(this._lastLog.timeout);
    const p2 = this._lastLog.time && o.date ? o.date.getTime() - this._lastLog.time.getTime() : 0;
    if (this._lastLog.time = o.date, p2 < this.options.throttle)
      try {
        const a = JSON.stringify([
          o.type,
          o.tag,
          o.args
        ]), c = this._lastLog.serialized === a;
        if (this._lastLog.serialized = a, c && (this._lastLog.count = (this._lastLog.count || 0) + 1, this._lastLog.count > this.options.throttleMin)) {
          this._lastLog.timeout = setTimeout(
            n,
            this.options.throttle
          );
          return;
        }
      } catch {
      }
    n(true);
  }
  _log(t) {
    for (const e of this.options.reporters)
      e.log(t, {
        options: this.options
      });
  }
}
function u(s, t = {}, e = 3) {
  return s === void 0 ? e : typeof s == "number" ? s : t[s] && t[s].level !== void 0 ? t[s].level : e;
}
i.prototype.add = i.prototype.addReporter;
i.prototype.remove = i.prototype.removeReporter;
i.prototype.clear = i.prototype.removeReporter;
i.prototype.withScope = i.prototype.withTag;
i.prototype.mock = i.prototype.mockTypes;
i.prototype.pause = i.prototype.pauseLogs;
i.prototype.resume = i.prototype.resumeLogs;
function b(s = {}) {
  return new i(s);
}
class S {
  options;
  defaultColor;
  levelColorMap;
  typeColorMap;
  constructor(t) {
    this.options = { ...t }, this.defaultColor = "#7f8c8d", this.levelColorMap = {
      0: "#c0392b",
      // Red
      1: "#f39c12",
      // Yellow
      3: "#00BCD4"
      // Cyan
    }, this.typeColorMap = {
      success: "#2ecc71"
      // Green
    };
  }
  _getLogFn(t) {
    return t < 1 ? console.__error || console.error : t === 1 ? console.__warn || console.warn : console.__log || console.log;
  }
  log(t) {
    const e = this._getLogFn(t.level), r = t.type === "log" ? "" : t.type, o = t.tag || "", p2 = `
      background: ${this.typeColorMap[t.type] || this.levelColorMap[t.level] || this.defaultColor};
      border-radius: 0.5em;
      color: white;
      font-weight: bold;
      padding: 2px 0.5em;
    `, a = `%c${[o, r].filter(Boolean).join(":")}`;
    typeof t.args[0] == "string" ? e(
      `${a}%c ${t.args[0]}`,
      p2,
      // Empty string as style resets to default console style
      "",
      ...t.args.slice(1)
    ) : e(a, p2, ...t.args);
  }
}
function C$1(s = {}) {
  return b({
    reporters: s.reporters || [new S({})],
    prompt(e, r = {}) {
      return r.type === "confirm" ? Promise.resolve(confirm(e)) : Promise.resolve(prompt(e));
    },
    ...s
  });
}
const F = C$1();
const y = (o) => {
  const e = o.getAttribute("data-cdrm-group");
  if (e)
    return [e, o];
  const t = o.parentElement;
  if (t)
    return y(t);
}, p = (o) => {
  const e = o.getAttribute("data-cdrm-idx");
  if (typeof e < "u" && e !== null)
    return parseInt(e);
}, T = (o) => {
  const e = o.currentTarget, t = p(e);
  if (typeof t > "u") {
    console.error("Could not find tab index.");
    return;
  }
  const r = y(e);
  if (!r) {
    console.error("Could not find proper parent element.");
    return;
  }
  const [a, n] = r, u2 = n.getAttribute("data-cdrm-focused-idx");
  if (typeof u2 > "u") {
    console.error("Could not found TabGroup focused index.");
    return;
  }
  const c = parseInt(u2), i2 = n.querySelectorAll(".cdrm-tab-subtle-border");
  for (let d2 = 0; d2 < i2.length; d2++) {
    const s = i2.item(d2), h2 = s.classList.values().toArray().filter((l2) => l2.startsWith("bg-"));
    for (const l2 of h2)
      s.classList.remove(l2);
    if (d2 === t) {
      const l2 = e.querySelector(
        ".cdrm-tab-subtle-border"
      );
      if (l2) {
        l2.style.transformOrigin = c < t ? "left" : "right", l2.classList.remove("bg-transparent"), l2.classList.remove("scale-x-0");
        const b2 = n.getAttribute("data-cdrm-emphasis");
        b2 && l2.classList.add(`bg-emphasis-${b2}`);
      }
    } else
      s.style.transformOrigin = c > t ? "left" : "right", s.classList.add("bg-transparent"), s.classList.add("scale-x-0");
  }
  n.setAttribute("data-cdrm-focused-idx", `${t}`);
  const m2 = n.getElementsByClassName("cdrm-tab-group-item");
  for (let d2 = 0; d2 < m2.length; d2++) {
    const s = m2.item(d2);
    s.getAttribute("data-cdrm-group") === a && (s.classList.remove("relative"), s.classList.add("absolute"), s.classList.remove("overflow-y-hidden"), d2 === t ? s.style.opacity = "1" : s.style.opacity = "0");
  }
}, I = (o) => {
  const t = o.currentTarget.getAttribute("data-cdrm-copy-for");
  if (!t) {
    F.error(
      "Failed to find a valid targetID on the code block.",
      o.currentTarget
    );
    return;
  }
  const r = document.getElementById(t);
  if (!r)
    return;
  window.navigator.clipboard.writeText(r.querySelector("pre")?.innerText ?? "").catch((n) => {
    F.error("Copy error: ", n);
  });
  const a = document.querySelector(
    `div[data-cdrm-codeblock="${t}"]`
  );
  if (a) {
    const n = a.getAttribute("data-cdrm-lang");
    window.dispatchEvent(
      new CustomEvent("cdrm-codeblock-copy", {
        detail: {
          lang: n
        }
      })
    );
  }
}, k = () => {
  F.info("Toggle copy icon here.");
};
function C(o) {
  const e = o.querySelector(
    ".cdrm-admon-body-container"
  );
  if (!e)
    return;
  const t = o.querySelector(".cdrm-admon-body");
  if (!t)
    return;
  t.style.transition = "max-height 500ms ease-in-out, height 500ms ease-in-out", t.style.height = "auto";
  const r = t.getBoundingClientRect().height;
  e.style.maxHeight = `${r}px`;
}
const E = (o) => {
  function e(c) {
    c.setAttribute("data-cdrm-folded", "false"), C(c);
  }
  function t(c) {
    const i2 = c.querySelector(
      ".cdrm-admon-body-container"
    );
    i2 && (i2.style.maxHeight = "0px", c.setAttribute("data-cdrm-folded", "true"));
  }
  const r = o.currentTarget.parentElement, a = r.getAttribute("data-cdrm-folded") === "true", n = r.getAttribute("data-cdrm-foldable") === "true";
  if (!r.querySelector(".cdrm-admon-body-container")) {
    console.error("Could not find admonition body");
    return;
  }
  n && (a ? e(r) : t(r));
};
function g(o) {
  const e = o.getAttribute("data-cdrm-focused-idx");
  if (typeof e > "u" || e === null) {
    console.error("No focused index found for the specified tab group.");
    return;
  }
  const t = parseInt(e), r = o.querySelector(".cdrm-tab-group-body-container");
  if (!r) {
    console.error("Could not locate tab group body container.");
    return;
  }
  const a = r.getElementsByClassName("cdrm-tab-group-item");
  for (let i2 = 0; i2 < a.length; i2++) {
    const m2 = a.item(i2);
    m2.style.transition = "transform 0.3s ease-in-out", m2.style.transform = `translateX(-${t === 0 ? 0 : t / a.length}%)`;
  }
  const n = o.getAttribute("data-cdrm-group");
  if (!n) {
    console.error("Could not locate tab group id. Can't continue.");
    return;
  }
  const u2 = r.querySelector(`#tab-${n}-${e}`);
  if (!u2) {
    console.error("Cold not locate focused tab. Cannot continue with transition.");
    return;
  }
  const c = Math.min(u2.getBoundingClientRect().height, 450);
  r.style.transition = "height 0.3s ease-in-out", r.style.overflowY = c === 450 ? "auto" : "hidden", r.style.height = `${c}px`;
}
const B = () => {
  const o = document.getElementsByClassName("cdrm-tab-group");
  for (let e = 0; e < o.length; e++) {
    const t = o.item(e);
    t && g(t);
  }
}, L = () => {
  const o = document.getElementsByClassName("cdrm-tab-group");
  for (let e = 0; e < o.length; e++) {
    const t = o.item(e);
    g(t), new MutationObserver(() => {
      g(t);
    }).observe(t, {
      attributes: true,
      attributeFilter: ["data-cdrm-focused-idx"]
    });
  }
}, x = (o) => {
  const e = o.currentTarget, t = document.querySelector(`a[data-cdrm-for="${e.id}"]`);
  if (!t) {
    console.error("Could not find footer for footnote anchor");
    return;
  }
  t.scrollIntoView({
    behavior: "smooth"
  });
}, v = (o) => {
  const t = o.currentTarget.getAttribute("data-cdrm-for");
  if (!t) {
    console.error("Could not find anchor id.");
    return;
  }
  const r = document.getElementById(t);
  if (!r) {
    console.error("Could not find an assocated anchor the footnote footer.");
    return;
  }
  r.scrollIntoView({
    behavior: "smooth"
  });
}, w = () => {
  const o = document.getElementsByClassName("cdrm-footnote");
  for (let t = 0; t < o.length; t++) {
    const r = o.item(t);
    r && r.addEventListener("click", v);
  }
  const e = document.getElementsByClassName("cdrm-footnote-anchor");
  for (let t = 0; t < e.length; t++) {
    const r = e.item(t);
    r && r.addEventListener("click", x);
  }
};
const methods_es = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  handleConundrumAdmonitionHeight: C,
  handleConundrumTabClick: T,
  onAdmonitionHeadingClick: E,
  onCodeBlockContainerClick: k,
  onCopyCodeBlockClick: I,
  onFootnotesLoad: w,
  onTabLoad: L,
  onTabResize: B
}, Symbol.toStringTag, { value: "Module" }));
export {
  applyCopyConundrumCodeBlockListeners
};
//# sourceMappingURL=code_block.es.js.map
