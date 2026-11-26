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
    const p = s[n];
    p != null && (Array.isArray(p) && Array.isArray(o[n]) ? o[n] = [...p, ...o[n]] : f(p) && f(o[n]) ? o[n] = g$1(
      p,
      o[n],
      (e ? `${e}.` : "") + n.toString()
    ) : o[n] = p);
  }
  return o;
}
function m(s) {
  return (...t) => (
    // eslint-disable-next-line unicorn/no-array-reduce
    t.reduce((e, r) => g$1(e, r, ""), {})
  );
}
const w = m();
function v(s) {
  return Object.prototype.toString.call(s) === "[object Object]";
}
function L$1(s) {
  return !(!v(s) || !s.message && !s.args || s.stack);
}
let h = false;
const y = [];
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
    this.options = w(
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
    const t = y.splice(0);
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
        y.push([this, t, r, e]);
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
    const p = this._lastLog.time && o.date ? o.date.getTime() - this._lastLog.time.getTime() : 0;
    if (this._lastLog.time = o.date, p < this.options.throttle)
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
    const e = this._getLogFn(t.level), r = t.type === "log" ? "" : t.type, o = t.tag || "", p = `
      background: ${this.typeColorMap[t.type] || this.levelColorMap[t.level] || this.defaultColor};
      border-radius: 0.5em;
      color: white;
      font-weight: bold;
      padding: 2px 0.5em;
    `, a = `%c${[o, r].filter(Boolean).join(":")}`;
    typeof t.args[0] == "string" ? e(
      `${a}%c ${t.args[0]}`,
      p,
      // Empty string as style resets to default console style
      "",
      ...t.args.slice(1)
    ) : e(a, p, ...t.args);
  }
}
function C(s = {}) {
  return b({
    reporters: s.reporters || [new S({})],
    prompt(e, r = {}) {
      return r.type === "confirm" ? Promise.resolve(confirm(e)) : Promise.resolve(prompt(e));
    },
    ...s
  });
}
C();
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
const L = () => {
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
};
(() => {
  L();
})();
//# sourceMappingURL=footnotes.es.js.map
