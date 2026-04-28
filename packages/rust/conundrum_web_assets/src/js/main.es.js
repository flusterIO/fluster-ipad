function C(a) {
  const s = a.querySelector(".cdrm-admon-body-container"), t = a.querySelector(".cdrm-admon-body");
  t.style.height = "auto", t.style.transition = "max-height 500ms ease-in-out";
  const n = t.getBoundingClientRect().height;
  s.style.maxHeight = `${n}px`;
}
function y() {
  function a(e) {
    e.setAttribute("data-cdrm-folded", "false"), C(e);
  }
  function s(e) {
    const o = e.querySelector(".cdrm-admon-body-container");
    o.style.maxHeight = "0px", e.setAttribute("data-cdrm-folded", "true");
  }
  let t = document.getElementsByClassName("cdrm-admon-title-group");
  for (var n = 0; n < t.length; n++)
    t.item(n)?.addEventListener("click", (o) => {
      const r = o.currentTarget.parentElement;
      let d = r.getAttribute("data-cdrm-folded") === "true", u = r.getAttribute("data-cdrm-foldable") === "true";
      r.querySelector(".cdrm-admon-body-container") && u && (d ? a(r) : s(r));
    });
}
function f() {
  const a = document.getElementsByClassName("cdrm-admon");
  for (var s = 0; s < a.length; s++) {
    const t = a.item(s);
    t.getAttribute("data-cdrm-folded") === "false" && t.getAttribute("data-cdrm-foldable") === "true" && C(t);
  }
}
y(), window.addEventListener("resize", f), window.addEventListener("cdrm-manual-resize", f), window.addEventListener("cdrm-content-loaded", y);
const p = () => {
  function a(e) {
    const r = e.currentTarget.getAttribute("data-cdrm-copy-for"), d = document.getElementById(r);
    d && (window.navigator.clipboard.writeText(d.querySelector("pre")?.innerText ?? ""), window.dispatchEvent(new CustomEvent("cdrm-codeblock-copy")));
  }
  const s = document.getElementsByClassName("cdrm-codeblock");
  for (var t = 0; t < s.length; t++) {
    const e = s.item(t);
    e.addEventListener("mouseenter", (o) => {
      o.target.classList.add("hovered");
    }), e.addEventListener("mouseleave", (o) => {
      o.target.classList.add("hovered");
    });
  }
  const n = document.getElementsByClassName("cdrm-codeblock-icon");
  for (var t = 0; t < n.length; t++)
    n.item(t).addEventListener("click", a);
};
p(), window.addEventListener("cdrm-content-loaded", p);
function v() {
  function a(n) {
    const e = n.currentTarget.parentElement.parentElement;
    if (!e)
      return;
    const o = e.getAttribute("data-cdrm-emphasis");
    let r = e.querySelectorAll(".cdrm-tab-subtle-border");
    const d = parseInt(n.currentTarget.getAttribute("data-cdrm-idx")), u = e.getAttribute("data-cdrm-group"), l = parseInt(e.getAttribute("data-cdrm-focused-idx"));
    for (var i = 0; i < r.length; i++) {
      const g = r.item(i);
      let m = g.classList.values().toArray().filter((c) => c.startsWith("bg-"));
      for (const c of m)
        g.classList.remove(c);
      if (i === d) {
        let c = n.currentTarget.querySelector(".cdrm-tab-subtle-border");
        c && (c.style.transformOrigin = l < d ? "left" : "right", c.classList.remove("bg-transparent"), c.classList.remove("scale-x-0"), c.classList.add(`bg-emphasis-${o}`));
      } else
        g.style.transformOrigin = l > d ? "left" : "right", g.classList.add("bg-transparent"), g.classList.add("scale-x-0");
    }
    e.setAttribute("data-cdrm-focused-idx", `${d}`);
    const E = document.getElementsByClassName("cdrm-tab-group-item");
    for (var i = 0; i < E.length; i++) {
      const m = E.item(i);
      m.getAttribute("data-cdrm-group") === u && (m.style.transform = `translateX(${(i - d) * 100}%)`, i === d ? m.style.opacity = "1" : m.style.opacity = "0");
    }
  }
  const s = document.getElementsByClassName("cdrm-tab-btn");
  for (var t = 0; t < s.length; t++)
    s.item(t).addEventListener("click", a);
}
function w() {
  function a(e) {
    const o = e.querySelectorAll(".cdrm-tab-group-item");
    for (var r = 0; r < o.length; r++) {
      const d = o.item(r);
      d && (d.style.position = "absolute");
    }
  }
  function s(e) {
    const o = parseInt(e.getAttribute("data-cdrm-focused-idx")), r = e.getAttribute("data-cdrm-group"), d = e.querySelector(`#tab-${r}-${o}`);
    if (d) {
      const u = d.getBoundingClientRect().height, l = e.querySelector(`#tab-body-wrapper-${r}`);
      l && (l.style.transition = "height 0.3s ease-in-out", l.style.height = `${Math.min(u, 450)}px`);
    }
  }
  const t = document.getElementsByClassName("cdrm-tab-group");
  for (var n = 0; n < t.length; n++) {
    const e = t.item(n);
    new MutationObserver(() => {
      s(e);
    }).observe(e, {
      attributes: !0,
      attributeFilter: ["data-cdrm-focused-idx"]
    }), s(e), a(e);
  }
}
v(), window.addEventListener("resize", w), window.addEventListener("cdrm-content-loaded", v);
const b = () => {
  w(), f();
}, h = () => {
  f(), w(), y(), p(), v();
}, L = () => {
  h(), b(), window.addEventListener("cdrm-content-loaded", h), window.addEventListener("resize", b), window.addEventListener("cdrm-manual-resize", b), console.info("Initialized Conundrum glue code...");
}, A = () => {
  window.removeEventListener("cdrm-content-loaded", h), window.removeEventListener("resize", b), window.removeEventListener("cdrm-manual-resize", b);
};
export {
  A as cleanupConundrumStaticWebAssets,
  L as initializeConundrumStaticWebAssets,
  h as onConunundrumContentLoaded
};
//# sourceMappingURL=main.es.js.map
