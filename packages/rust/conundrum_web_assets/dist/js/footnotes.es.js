const c = {
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
}, C = {
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
function h(o) {
  if (o === null || typeof o != "object")
    return false;
  const t = Object.getPrototypeOf(o);
  return t !== null && t !== Object.prototype && Object.getPrototypeOf(t) !== null || Symbol.iterator in o ? false : Symbol.toStringTag in o ? Object.prototype.toString.call(o) === "[object Module]" : true;
}
function b(o, t, e = ".", s) {
  if (!h(t))
    return b(o, {}, e);
  const r = Object.assign({}, t);
  for (const n in o) {
    if (n === "__proto__" || n === "constructor")
      continue;
    const l = o[n];
    l != null && (Array.isArray(l) && Array.isArray(r[n]) ? r[n] = [...l, ...r[n]] : h(l) && h(r[n]) ? r[n] = b(
      l,
      r[n],
      (e ? `${e}.` : "") + n.toString()
    ) : r[n] = l);
  }
  return r;
}
function S(o) {
  return (...t) => (
    // eslint-disable-next-line unicorn/no-array-reduce
    t.reduce((e, s) => b(e, s, ""), {})
  );
}
const T = S();
function k(o) {
  return Object.prototype.toString.call(o) === "[object Object]";
}
function x(o) {
  return !(!k(o) || !o.message && !o.args || o.stack);
}
let m = false;
const L = [];
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
    const e = t.types || C;
    this.options = T(
      {
        ...t,
        defaults: { ...t.defaults },
        level: y(t.level, e),
        reporters: [...t.reporters || []]
      },
      {
        types: C,
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
      const r = {
        type: s,
        ...this.options.defaults,
        ...e[s]
      };
      this[s] = this._wrapLogFn(r), this[s].raw = this._wrapLogFn(
        r,
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
      for (const s in this.options.types)
        this[s] = e(s, this.options.types[s]) || this[s], this[s].raw = this[s];
  }
  _wrapLogFn(t, e) {
    return (...s) => {
      if (m) {
        L.push([this, t, s, e]);
        return;
      }
      return this._logFn(t, s, e);
    };
  }
  _logFn(t, e, s) {
    if ((t.level || 0) > this.level)
      return false;
    const r = {
      date: /* @__PURE__ */ new Date(),
      args: [],
      ...t,
      level: y(t.level, this.options.types)
    };
    !s && e.length === 1 && x(e[0]) ? Object.assign(r, e[0]) : r.args = [...e], r.message && (r.args.unshift(r.message), delete r.message), r.additional && (Array.isArray(r.additional) || (r.additional = r.additional.split(`
`)), r.args.push(`
` + r.additional.join(`
`)), delete r.additional), r.type = typeof r.type == "string" ? r.type.toLowerCase() : "log", r.tag = typeof r.tag == "string" ? r.tag : "";
    const n = (a = false) => {
      const d = (this._lastLog.count || 0) - this.options.throttleMin;
      if (this._lastLog.object && d > 0) {
        const g = [...this._lastLog.object.args];
        d > 1 && g.push(`(repeated ${d} times)`), this._log({ ...this._lastLog.object, args: g }), this._lastLog.count = 1;
      }
      a && (this._lastLog.object = r, this._log(r));
    };
    clearTimeout(this._lastLog.timeout);
    const l = this._lastLog.time && r.date ? r.date.getTime() - this._lastLog.time.getTime() : 0;
    if (this._lastLog.time = r.date, l < this.options.throttle)
      try {
        const a = JSON.stringify([
          r.type,
          r.tag,
          r.args
        ]), d = this._lastLog.serialized === a;
        if (this._lastLog.serialized = a, d && (this._lastLog.count = (this._lastLog.count || 0) + 1, this._lastLog.count > this.options.throttleMin)) {
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
function y(o, t = {}, e = 3) {
  return o === void 0 ? e : typeof o == "number" ? o : t[o] && t[o].level !== void 0 ? t[o].level : e;
}
i.prototype.add = i.prototype.addReporter;
i.prototype.remove = i.prototype.removeReporter;
i.prototype.clear = i.prototype.removeReporter;
i.prototype.withScope = i.prototype.withTag;
i.prototype.mock = i.prototype.mockTypes;
i.prototype.pause = i.prototype.pauseLogs;
i.prototype.resume = i.prototype.resumeLogs;
function F(o = {}) {
  return new i(o);
}
class I {
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
    const e = this._getLogFn(t.level), s = t.type === "log" ? "" : t.type, r = t.tag || "", l = `
      background: ${this.typeColorMap[t.type] || this.levelColorMap[t.level] || this.defaultColor};
      border-radius: 0.5em;
      color: white;
      font-weight: bold;
      padding: 2px 0.5em;
    `, a = `%c${[r, s].filter(Boolean).join(":")}`;
    typeof t.args[0] == "string" ? e(
      `${a}%c ${t.args[0]}`,
      l,
      // Empty string as style resets to default console style
      "",
      ...t.args.slice(1)
    ) : e(a, l, ...t.args);
  }
}
function B(o = {}) {
  return F({
    reporters: o.reporters || [new I({})],
    prompt(e, s = {}) {
      return s.type === "confirm" ? Promise.resolve(confirm(e)) : Promise.resolve(prompt(e));
    },
    ...o
  });
}
B();
function v(o) {
  const t = parseInt(
    /* eslint-disable-next-line  -- It'll be there... I put it there. */
    o.getAttribute("data-cdrm-focused-idx")
  ), e = o.getAttribute("data-cdrm-group");
  if (!e) {
    console.warn("Compiler Error: Found a tab group without a valid group id.");
    return;
  }
  const s = o.querySelector(
    `#tab-${e}-${t}`
  );
  if (s) {
    const r = s.getBoundingClientRect().height, n = o.querySelector(
      `#tab-body-wrapper-${e}`
    );
    n ? (n.style.transition = "height 0.3s ease-in-out", n.style.height = `${Math.min(r, 450)}px`, n.style.overflowY = r > 450 ? "auto" : "hidden") : console.error("Could not find tab body wrapper.");
  } else
    console.error("Could not find focused body");
}
const P = () => {
  const o = document.getElementsByClassName("cdrm-tab-group");
  for (let t = 0; t < o.length; t++) {
    const e = o.item(t);
    v(e), new MutationObserver(() => {
      v(e);
    }).observe(e, {
      attributes: true,
      attributeFilter: ["data-cdrm-focused-idx"]
    });
  }
};
(() => {
  P();
})();
//# sourceMappingURL=footnotes.es.js.map
