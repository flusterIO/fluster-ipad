const applyCopyConundrumCodeBlockListeners = () => {
  const ems = document.getElementsByClassName("cdrm-codeblock");
  for (var i = 0; i < ems.length; i++) {
    const item = ems.item(i);
    item.addEventListener("mouseenter", (e) => {
      e.target.classList.add("hovered");
    });
    item.addEventListener("mouseleave", (e) => {
      e.target.classList.add("hovered");
    });
  }
  const icons = document.getElementsByClassName("cdrm-codeblock-icon");
  for (var i = 0; i < icons.length; i++) {
    const item = icons.item(i);
    item.addEventListener("click", async (e) => {
      Promise.resolve().then(() => methods_es).then((a2) => a2.onCopyCodeBlockClick(e));
    });
  }
};
(() => {
  applyCopyConundrumCodeBlockListeners();
  window.addEventListener("cdrm-content-loaded", applyCopyConundrumCodeBlockListeners);
})();
const B = (s) => {
  const t = s.currentTarget, e = t.parentElement?.parentElement;
  if (!e) {
    console.error("Could not find proper parent element.");
    return;
  }
  const r = t.getAttribute("data-cdrm-idx");
  if (typeof r > "u") {
    console.error("Could not find tab index.");
    return;
  }
  const o = parseInt(r), i = e.getAttribute("data-cdrm-group"), l = e.getAttribute("data-cdrm-focused-idx");
  if (typeof l > "u") {
    console.error("Could not found TabGroup focused index.");
    return;
  }
  const n = parseInt(l), d = e.querySelectorAll(".cdrm-tab-subtle-border");
  for (let p = 0; p < d.length; p++) {
    const u = d.item(p), C = u.classList.values().toArray().filter((f) => f.startsWith("bg-"));
    for (const f of C)
      u.classList.remove(f);
    if (p === o) {
      const f = t.querySelector(
        ".cdrm-tab-subtle-border"
      );
      if (f) {
        f.style.transformOrigin = n < o ? "left" : "right", f.classList.remove("bg-transparent"), f.classList.remove("scale-x-0");
        const v = e.getAttribute("data-cdrm-emphasis");
        v && f.classList.add(`bg-emphasis-${v}`);
      }
    } else
      u.style.transformOrigin = n > o ? "left" : "right", u.classList.add("bg-transparent"), u.classList.add("scale-x-0");
  }
  e.setAttribute("data-cdrm-focused-idx", `${o}`);
  const g = document.getElementsByClassName("cdrm-tab-group-item");
  for (let p = 0; p < g.length; p++) {
    const u = g.item(p);
    u.getAttribute("data-cdrm-group") === i && (u.style.transform = `translateX(${(p - o) * 100}%)`, p === o ? u.style.opacity = "1" : u.style.opacity = "0");
  }
}, c = {
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
}, w = {
  // Silent
  silent: {
    level: -1
  },
  // Level 0
  fatal: {
    level: c.fatal
  },
  error: {
    level: c.error
  },
  // Level 1
  warn: {
    level: c.warn
  },
  // Level 2
  log: {
    level: c.log
  },
  // Level 3
  info: {
    level: c.info
  },
  success: {
    level: c.success
  },
  fail: {
    level: c.fail
  },
  ready: {
    level: c.info
  },
  start: {
    level: c.info
  },
  box: {
    level: c.info
  },
  // Level 4
  debug: {
    level: c.debug
  },
  // Level 5
  trace: {
    level: c.trace
  },
  // Verbose
  verbose: {
    level: c.verbose
  }
};
function h(s) {
  if (s === null || typeof s != "object")
    return false;
  const t = Object.getPrototypeOf(s);
  return t !== null && t !== Object.prototype && Object.getPrototypeOf(t) !== null || Symbol.iterator in s ? false : Symbol.toStringTag in s ? Object.prototype.toString.call(s) === "[object Module]" : true;
}
function b(s, t, e = ".", r) {
  if (!h(t))
    return b(s, {}, e);
  const o = Object.assign({}, t);
  for (const i in s) {
    if (i === "__proto__" || i === "constructor")
      continue;
    const l = s[i];
    l != null && (Array.isArray(l) && Array.isArray(o[i]) ? o[i] = [...l, ...o[i]] : h(l) && h(o[i]) ? o[i] = b(
      l,
      o[i],
      (e ? `${e}.` : "") + i.toString()
    ) : o[i] = l);
  }
  return o;
}
function S(s) {
  return (...t) => (
    // eslint-disable-next-line unicorn/no-array-reduce
    t.reduce((e, r) => b(e, r, ""), {})
  );
}
const A = S();
function T(s) {
  return Object.prototype.toString.call(s) === "[object Object]";
}
function k(s) {
  return !(!T(s) || !s.message && !s.args || s.stack);
}
let m = false;
const L = [];
class a {
  options;
  _lastLog;
  _mockFn;
  /**
   * Creates an instance of Consola with specified options or defaults.
   *
   * @param {Partial<ConsolaOptions>} [options={}] - Configuration options for the Consola instance.
   */
  constructor(t = {}) {
    const e = t.types || w;
    this.options = A(
      {
        ...t,
        defaults: { ...t.defaults },
        level: y(t.level, e),
        reporters: [...t.reporters || []]
      },
      {
        types: w,
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
    this.options.level = y(
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
    const e = new a({
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
    m = true;
  }
  /**
   * Resumes logging, processing any queued logs.
   */
  resumeLogs() {
    m = false;
    const t = L.splice(0);
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
      if (m) {
        L.push([this, t, r, e]);
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
      level: y(t.level, this.options.types)
    };
    !r && e.length === 1 && k(e[0]) ? Object.assign(o, e[0]) : o.args = [...e], o.message && (o.args.unshift(o.message), delete o.message), o.additional && (Array.isArray(o.additional) || (o.additional = o.additional.split(`
`)), o.args.push(`
` + o.additional.join(`
`)), delete o.additional), o.type = typeof o.type == "string" ? o.type.toLowerCase() : "log", o.tag = typeof o.tag == "string" ? o.tag : "";
    const i = (n = false) => {
      const d = (this._lastLog.count || 0) - this.options.throttleMin;
      if (this._lastLog.object && d > 0) {
        const g = [...this._lastLog.object.args];
        d > 1 && g.push(`(repeated ${d} times)`), this._log({ ...this._lastLog.object, args: g }), this._lastLog.count = 1;
      }
      n && (this._lastLog.object = o, this._log(o));
    };
    clearTimeout(this._lastLog.timeout);
    const l = this._lastLog.time && o.date ? o.date.getTime() - this._lastLog.time.getTime() : 0;
    if (this._lastLog.time = o.date, l < this.options.throttle)
      try {
        const n = JSON.stringify([
          o.type,
          o.tag,
          o.args
        ]), d = this._lastLog.serialized === n;
        if (this._lastLog.serialized = n, d && (this._lastLog.count = (this._lastLog.count || 0) + 1, this._lastLog.count > this.options.throttleMin)) {
          this._lastLog.timeout = setTimeout(
            i,
            this.options.throttle
          );
          return;
        }
      } catch {
      }
    i(true);
  }
  _log(t) {
    for (const e of this.options.reporters)
      e.log(t, {
        options: this.options
      });
  }
}
function y(s, t = {}, e = 3) {
  return s === void 0 ? e : typeof s == "number" ? s : t[s] && t[s].level !== void 0 ? t[s].level : e;
}
a.prototype.add = a.prototype.addReporter;
a.prototype.remove = a.prototype.removeReporter;
a.prototype.clear = a.prototype.removeReporter;
a.prototype.withScope = a.prototype.withTag;
a.prototype.mock = a.prototype.mockTypes;
a.prototype.pause = a.prototype.pauseLogs;
a.prototype.resume = a.prototype.resumeLogs;
function x(s = {}) {
  return new a(s);
}
class F {
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
    const e = this._getLogFn(t.level), r = t.type === "log" ? "" : t.type, o = t.tag || "", l = `
      background: ${this.typeColorMap[t.type] || this.levelColorMap[t.level] || this.defaultColor};
      border-radius: 0.5em;
      color: white;
      font-weight: bold;
      padding: 2px 0.5em;
    `, n = `%c${[o, r].filter(Boolean).join(":")}`;
    typeof t.args[0] == "string" ? e(
      `${n}%c ${t.args[0]}`,
      l,
      // Empty string as style resets to default console style
      "",
      ...t.args.slice(1)
    ) : e(n, l, ...t.args);
  }
}
function I(s = {}) {
  return x({
    reporters: s.reporters || [new F({})],
    prompt(e, r = {}) {
      return r.type === "confirm" ? Promise.resolve(confirm(e)) : Promise.resolve(prompt(e));
    },
    ...s
  });
}
const _ = I(), q = (s) => {
  const e = s.currentTarget.getAttribute("data-cdrm-copy-for");
  if (!e) {
    _.error(
      "Failed to find a valid targetID on the code block.",
      s.currentTarget
    );
    return;
  }
  const r = document.getElementById(e);
  if (!r)
    return;
  window.navigator.clipboard.writeText(r.querySelector("pre")?.innerText ?? "").catch((i) => {
    _.error("Copy error: ", i);
  });
  const o = document.querySelector(
    `div[data-cdrm-codeblock="${e}"]`
  );
  if (o) {
    const i = o.getAttribute("data-cdrm-lang");
    window.dispatchEvent(
      new CustomEvent("cdrm-codeblock-copy", {
        detail: {
          lang: i
        }
      })
    );
  }
}, E = () => {
  _.info("Toggle copy icon here.");
};
function $(s) {
  const t = s.querySelector(
    ".cdrm-admon-body-container"
  );
  if (!t)
    return;
  const e = s.querySelector(".cdrm-admon-body");
  if (!e)
    return;
  e.style.height = "auto", e.style.transition = "max-height 500ms ease-in-out";
  const r = e.getBoundingClientRect().height;
  t.style.maxHeight = `${r}px`;
}
const M = (s) => {
  function t(n) {
    n.setAttribute("data-cdrm-folded", "false"), $(n);
  }
  function e(n) {
    const d = n.querySelector(
      ".cdrm-admon-body-container"
    );
    d && (d.style.maxHeight = "0px", n.setAttribute("data-cdrm-folded", "true"));
  }
  const r = s.currentTarget.parentElement;
  console.log("container: ", r);
  const o = r.getAttribute("data-cdrm-folded") === "true", i = r.getAttribute("data-cdrm-foldable") === "true";
  r.querySelector(".cdrm-admon-body-container") && i && (o ? t(r) : e(r));
}, j = () => {
  function s(e) {
    const r = parseInt(
      /* eslint-disable-next-line  -- It'll be there... I put it there. */
      e.getAttribute("data-cdrm-focused-idx")
    ), o = e.getAttribute("data-cdrm-group");
    if (!o) {
      console.warn(
        "Compiler Error: Found a tab group without a valid group id."
      );
      return;
    }
    const i = e.querySelector(
      `#tab-${o}-${r}`
    );
    if (i) {
      const l = i.getBoundingClientRect().height, n = e.querySelector(
        `#tab-body-wrapper-${o}`
      );
      n ? (n.style.transition = "height 0.3s ease-in-out", n.style.height = `${Math.min(l, 450)}px`, n.style.overflowY = l > 450 ? "auto" : "hidden") : console.error("Could not find tab body wrapper.");
    } else
      console.error("Could not find focused body");
  }
  const t = document.getElementsByClassName("cdrm-tab-group");
  for (let e = 0; e < t.length; e++) {
    const r = t.item(e);
    s(r), new MutationObserver(() => {
      s(r);
    }).observe(r, {
      attributes: true,
      attributeFilter: ["data-cdrm-focused-idx"]
    });
  }
};
const methods_es = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  handleConundrumAdmonitionHeight: $,
  handleConundrumTabClick: B,
  onAdmonitionHeadingClick: M,
  onCodeBlockContainerClick: E,
  onCopyCodeBlockClick: q,
  onTabLoad: j
}, Symbol.toStringTag, { value: "Module" }));
export {
  applyCopyConundrumCodeBlockListeners
};
//# sourceMappingURL=code_block.es.js.map
