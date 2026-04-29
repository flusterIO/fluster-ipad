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
      Promise.resolve().then(() => methods_es).then((a) => a.onCopyCodeBlockClick(e));
    });
  }
};
(() => {
  applyCopyConundrumCodeBlockListeners();
  window.addEventListener("cdrm-content-loaded", applyCopyConundrumCodeBlockListeners);
})();
const $ = (r) => {
  const t = r.currentTarget?.parentElement?.parentElement;
  if (!t)
    return;
  const e = t.getAttribute("data-cdrm-emphasis"), s = t.querySelectorAll(".cdrm-tab-subtle-border"), o = t.getAttribute("data-cdrm-idx");
  if (!o)
    return;
  const i = parseInt(o), a = t.getAttribute("data-cdrm-group"), l2 = t.getAttribute("data-cdrm-focused-idx");
  if (typeof l2 > "u")
    return;
  const p = parseInt(l2);
  for (let d = 0; d < s.length; d++) {
    const u = s.item(d), L = u.classList.values().toArray().filter((f) => f.startsWith("bg-"));
    for (const f of L)
      u.classList.remove(f);
    if (d === i) {
      const f = t.querySelector(".cdrm-tab-subtle-border");
      f && (f.style.transformOrigin = p < i ? "left" : "right", f.classList.remove("bg-transparent"), f.classList.remove("scale-x-0"), e && f.classList.add(`bg-emphasis-${e}`));
    } else
      u.style.transformOrigin = p > i ? "left" : "right", u.classList.add("bg-transparent"), u.classList.add("scale-x-0");
  }
  t.setAttribute("data-cdrm-focused-idx", `${i}`);
  const g = document.getElementsByClassName("cdrm-tab-group-item");
  for (let d = 0; d < g.length; d++) {
    const u = g.item(d);
    u.getAttribute("data-cdrm-group") === a && (u.style.transform = `translateX(${(d - i) * 100}%)`, d === i ? u.style.opacity = "1" : u.style.opacity = "0");
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
}, v = {
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
function h(r) {
  if (r === null || typeof r != "object")
    return false;
  const t = Object.getPrototypeOf(r);
  return t !== null && t !== Object.prototype && Object.getPrototypeOf(t) !== null || Symbol.iterator in r ? false : Symbol.toStringTag in r ? Object.prototype.toString.call(r) === "[object Module]" : true;
}
function _(r, t, e = ".", s) {
  if (!h(t))
    return _(r, {}, e);
  const o = Object.assign({}, t);
  for (const i in r) {
    if (i === "__proto__" || i === "constructor")
      continue;
    const a = r[i];
    a != null && (Array.isArray(a) && Array.isArray(o[i]) ? o[i] = [...a, ...o[i]] : h(a) && h(o[i]) ? o[i] = _(
      a,
      o[i],
      (e ? `${e}.` : "") + i.toString()
    ) : o[i] = a);
  }
  return o;
}
function C(r) {
  return (...t) => (
    // eslint-disable-next-line unicorn/no-array-reduce
    t.reduce((e, s) => _(e, s, ""), {})
  );
}
const A = C();
function S(r) {
  return Object.prototype.toString.call(r) === "[object Object]";
}
function k(r) {
  return !(!S(r) || !r.message && !r.args || r.stack);
}
let m = false;
const w = [];
class n {
  options;
  _lastLog;
  _mockFn;
  /**
   * Creates an instance of Consola with specified options or defaults.
   *
   * @param {Partial<ConsolaOptions>} [options={}] - Configuration options for the Consola instance.
   */
  constructor(t = {}) {
    const e = t.types || v;
    this.options = A(
      {
        ...t,
        defaults: { ...t.defaults },
        level: y(t.level, e),
        reporters: [...t.reporters || []]
      },
      {
        types: v,
        throttle: 1e3,
        throttleMin: 5,
        formatOptions: {
          date: true,
          colors: false,
          compact: true
        }
      }
    );
    for (const s in e) {
      const o = {
        type: s,
        ...this.options.defaults,
        ...e[s]
      };
      this[s] = this._wrapLogFn(o), this[s].raw = this._wrapLogFn(
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
    const e = new n({
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
    t && (t.__write || (t.__write = t.write), t.write = (s) => {
      this[e].raw(String(s).trim());
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
    const t = w.splice(0);
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
      for (const s in this.options.types)
        this[s] = e(s, this.options.types[s]) || this[s], this[s].raw = this[s];
  }
  _wrapLogFn(t, e) {
    return (...s) => {
      if (m) {
        w.push([this, t, s, e]);
        return;
      }
      return this._logFn(t, s, e);
    };
  }
  _logFn(t, e, s) {
    if ((t.level || 0) > this.level)
      return false;
    const o = {
      date: /* @__PURE__ */ new Date(),
      args: [],
      ...t,
      level: y(t.level, this.options.types)
    };
    !s && e.length === 1 && k(e[0]) ? Object.assign(o, e[0]) : o.args = [...e], o.message && (o.args.unshift(o.message), delete o.message), o.additional && (Array.isArray(o.additional) || (o.additional = o.additional.split(`
`)), o.args.push(`
` + o.additional.join(`
`)), delete o.additional), o.type = typeof o.type == "string" ? o.type.toLowerCase() : "log", o.tag = typeof o.tag == "string" ? o.tag : "";
    const i = (l2 = false) => {
      const p = (this._lastLog.count || 0) - this.options.throttleMin;
      if (this._lastLog.object && p > 0) {
        const g = [...this._lastLog.object.args];
        p > 1 && g.push(`(repeated ${p} times)`), this._log({ ...this._lastLog.object, args: g }), this._lastLog.count = 1;
      }
      l2 && (this._lastLog.object = o, this._log(o));
    };
    clearTimeout(this._lastLog.timeout);
    const a = this._lastLog.time && o.date ? o.date.getTime() - this._lastLog.time.getTime() : 0;
    if (this._lastLog.time = o.date, a < this.options.throttle)
      try {
        const l2 = JSON.stringify([
          o.type,
          o.tag,
          o.args
        ]), p = this._lastLog.serialized === l2;
        if (this._lastLog.serialized = l2, p && (this._lastLog.count = (this._lastLog.count || 0) + 1, this._lastLog.count > this.options.throttleMin)) {
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
function y(r, t = {}, e = 3) {
  return r === void 0 ? e : typeof r == "number" ? r : t[r] && t[r].level !== void 0 ? t[r].level : e;
}
n.prototype.add = n.prototype.addReporter;
n.prototype.remove = n.prototype.removeReporter;
n.prototype.clear = n.prototype.removeReporter;
n.prototype.withScope = n.prototype.withTag;
n.prototype.mock = n.prototype.mockTypes;
n.prototype.pause = n.prototype.pauseLogs;
n.prototype.resume = n.prototype.resumeLogs;
function T(r = {}) {
  return new n(r);
}
class x {
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
    const e = this._getLogFn(t.level), s = t.type === "log" ? "" : t.type, o = t.tag || "", a = `
      background: ${this.typeColorMap[t.type] || this.levelColorMap[t.level] || this.defaultColor};
      border-radius: 0.5em;
      color: white;
      font-weight: bold;
      padding: 2px 0.5em;
    `, l2 = `%c${[o, s].filter(Boolean).join(":")}`;
    typeof t.args[0] == "string" ? e(
      `${l2}%c ${t.args[0]}`,
      a,
      // Empty string as style resets to default console style
      "",
      ...t.args.slice(1)
    ) : e(l2, a, ...t.args);
  }
}
function F(r = {}) {
  return T({
    reporters: r.reporters || [new x({})],
    prompt(e, s = {}) {
      return s.type === "confirm" ? Promise.resolve(confirm(e)) : Promise.resolve(prompt(e));
    },
    ...r
  });
}
const b = F(), j = (r) => {
  const e = r.currentTarget.getAttribute("data-cdrm-copy-for");
  if (!e) {
    b.error("Failed to find a valid targetID on the code block.");
    return;
  }
  const s = document.getElementById(e);
  s && (window.navigator.clipboard.writeText(s.querySelector("pre")?.innerText ?? "").catch((o) => {
    b.error("Copy error: ", o);
  }), window.dispatchEvent(new CustomEvent("cdrm-codeblock-copy")));
}, B = () => {
  b.info("Toggle copy icon here.");
};
function I(r) {
  const t = r.querySelector(
    ".cdrm-admon-body-container"
  );
  if (!t)
    return;
  const e = r.querySelector(".cdrm-admon-body");
  if (!e)
    return;
  e.style.height = "auto", e.style.transition = "max-height 500ms ease-in-out";
  const s = e.getBoundingClientRect().height;
  t.style.maxHeight = `${s}px`;
}
const E = (r) => {
  function t(l2) {
    l2.setAttribute("data-cdrm-folded", "false"), I(l2);
  }
  function e(l2) {
    const p = l2.querySelector(
      ".cdrm-admon-body-container"
    );
    p && (p.style.maxHeight = "0px", l2.setAttribute("data-cdrm-folded", "true"));
  }
  console.log("Admonition title clicked!");
  const s = r.currentTarget.parentElement, o = s.getAttribute("data-cdrm-folded") === "true", i = s.getAttribute("data-cdrm-foldable") === "true";
  s.querySelector(".cdrm-admon-body-container") && i && (o ? t(s) : e(s));
};
const l = () => {
  function s(t) {
    const e = t.querySelectorAll(".cdrm-tab-group-item");
    for (let o = 0; o < e.length; o++) {
      const r = e.item(o);
      r && (r.style.position = "absolute");
    }
  }
  function n2(t) {
    const e = parseInt(
      /* eslint-disable-next-line  -- It'll be there... I put it there. */
      t.getAttribute("data-cdrm-focused-idx")
    ), o = t.getAttribute("data-cdrm-group");
    if (!o) {
      b.warn(
        "Compiler Error: Found a tab group without a valid group id."
      );
      return;
    }
    const r = t.querySelector(
      `#tab-${o}-${e}`
    );
    if (r) {
      const d = r.getBoundingClientRect().height, i = t.querySelector(
        `#tab-body-wrapper-${o}`
      );
      i && (i.style.transition = "height 0.3s ease-in-out", i.style.height = `${Math.min(d, 450)}px`);
    }
  }
  const a = document.getElementsByClassName("cdrm-tab-group");
  for (let t = 0; t < a.length; t++) {
    const e = a.item(t);
    new MutationObserver(() => {
      n2(e);
    }).observe(e, {
      attributes: true,
      attributeFilter: ["data-cdrm-focused-idx"]
    }), n2(e), s(e);
  }
};
const methods_es = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  handleConundrumAdmonitionHeight: I,
  handleConundrumTabClick: $,
  onAdmonitionHeadingClick: E,
  onCodeBlockContainerClick: B,
  onCopyCodeBlockClick: j,
  onTabResize: l
}, Symbol.toStringTag, { value: "Module" }));
export {
  applyCopyConundrumCodeBlockListeners
};
//# sourceMappingURL=code_block.es.js.map
