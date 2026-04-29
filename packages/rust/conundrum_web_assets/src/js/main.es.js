function handleConundrumAdmonitionHeight(container) {
  const bodyContainer = container.querySelector(".cdrm-admon-body-container");
  const body = container.querySelector(".cdrm-admon-body");
  body.style.height = "auto";
  body.style.transition = "max-height 500ms ease-in-out";
  const bodyHeight = body.getBoundingClientRect().height;
  bodyContainer.style.maxHeight = `${bodyHeight}px`;
}
function applyAdmonitionClickListeners() {
  function openAdmonition(container) {
    container.setAttribute("data-cdrm-folded", "false");
    handleConundrumAdmonitionHeight(container);
  }
  function closeAdmonition(container) {
    const bodyContainer = container.querySelector(".cdrm-admon-body-container");
    bodyContainer.style.maxHeight = "0px";
    container.setAttribute("data-cdrm-folded", "true");
  }
  let ems = document.getElementsByClassName("cdrm-admon-title-group");
  for (var i = 0; i < ems.length; i++) {
    const item = ems.item(i);
    item?.addEventListener("click", (e) => {
      const container = e.currentTarget.parentElement;
      let folded = container.getAttribute("data-cdrm-folded") === "true";
      let foldable = container.getAttribute("data-cdrm-foldable") === "true";
      const body = container.querySelector(".cdrm-admon-body-container");
      if (!body) {
        return;
      }
      if (foldable) {
        if (folded) {
          openAdmonition(container);
        } else {
          closeAdmonition(container);
        }
      }
    });
  }
}
function handleConundrumAdmonitionResize() {
  const ems = document.getElementsByClassName("cdrm-admon");
  for (var i = 0; i < ems.length; i++) {
    const item = ems.item(i);
    if (item.getAttribute("data-cdrm-folded") === "false" && item.getAttribute("data-cdrm-foldable") === "true") {
      handleConundrumAdmonitionHeight(item);
    }
  }
}
const applyCopyConundrumCodeBlockListeners = () => {
  function copyCodeblockCode(e) {
    let target = e.currentTarget;
    const targetId = target.getAttribute("data-cdrm-copy-for");
    const parentEm = document.getElementById(targetId);
    if (!parentEm) {
      return;
    }
    window.navigator.clipboard.writeText(parentEm.querySelector("pre")?.innerText ?? "");
    window.dispatchEvent(new CustomEvent("cdrm-codeblock-copy"));
  }
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
    item.addEventListener("click", copyCodeblockCode);
  }
};
(() => {
  applyCopyConundrumCodeBlockListeners();
  window.addEventListener("cdrm-content-loaded", applyCopyConundrumCodeBlockListeners);
})();
function addConundrumTabClickListeners() {
  function handleTabClick(e) {
    const em = e.currentTarget.parentElement.parentElement;
    if (!em) {
      return;
    }
    const emphasis = em.getAttribute("data-cdrm-emphasis");
    let tabs = em.querySelectorAll(".cdrm-tab-subtle-border");
    const clickedIndex = parseInt(e.currentTarget.getAttribute("data-cdrm-idx"));
    const groupId = em.getAttribute("data-cdrm-group");
    const lastFocusedIndex = parseInt(em.getAttribute("data-cdrm-focused-idx"));
    for (var i2 = 0; i2 < tabs.length; i2++) {
      const tab = tabs.item(i2);
      let bgClasses = tab.classList.values().toArray().filter((s) => s.startsWith("bg-"));
      for (const k of bgClasses) {
        tab.classList.remove(k);
      }
      if (i2 === clickedIndex) {
        let activeTabBorder = e.currentTarget.querySelector(".cdrm-tab-subtle-border");
        if (activeTabBorder) {
          activeTabBorder.style.transformOrigin = lastFocusedIndex < clickedIndex ? "left" : "right";
          activeTabBorder.classList.remove("bg-transparent");
          activeTabBorder.classList.remove("scale-x-0");
          activeTabBorder.classList.add(`bg-emphasis-${emphasis}`);
        }
      } else {
        tab.style.transformOrigin = lastFocusedIndex > clickedIndex ? "left" : "right";
        tab.classList.add("bg-transparent");
        tab.classList.add("scale-x-0");
      }
    }
    em.setAttribute("data-cdrm-focused-idx", `${clickedIndex}`);
    const allTabBodies = document.getElementsByClassName("cdrm-tab-group-item");
    for (var i2 = 0; i2 < allTabBodies.length; i2++) {
      const tabBody = allTabBodies.item(i2);
      if (tabBody.getAttribute("data-cdrm-group") === groupId) {
        tabBody.style.transform = `translateX(${(i2 - clickedIndex) * 100}%)`;
        if (i2 === clickedIndex) {
          tabBody.style.opacity = "1";
        } else {
          tabBody.style.opacity = "0";
        }
      }
    }
  }
  const ems = document.getElementsByClassName("cdrm-tab-btn");
  for (var i = 0; i < ems.length; i++) {
    const item = ems.item(i);
    item.addEventListener("click", handleTabClick);
  }
}
function handleConundrumTabGroupHeight() {
  function removeInitialRelativePositions(container) {
    const tabs = container.querySelectorAll(".cdrm-tab-group-item");
    for (var i2 = 0; i2 < tabs.length; i2++) {
      const item = tabs.item(i2);
      if (item) {
        item.style.position = "absolute";
      }
    }
  }
  function handleHeight(container) {
    const focusedIndex = parseInt(container.getAttribute("data-cdrm-focused-idx"));
    const groupId = container.getAttribute("data-cdrm-group");
    const focusedTabBody = container.querySelector(`#tab-${groupId}-${focusedIndex}`);
    if (focusedTabBody) {
      const h = focusedTabBody.getBoundingClientRect().height;
      const bodyWrapper = container.querySelector(`#tab-body-wrapper-${groupId}`);
      if (bodyWrapper) {
        bodyWrapper.style.transition = "height 0.3s ease-in-out";
        bodyWrapper.style.height = `${Math.min(h, 450)}px`;
      }
    }
  }
  const containers = document.getElementsByClassName("cdrm-tab-group");
  for (var i = 0; i < containers.length; i++) {
    const tabGroup = containers.item(i);
    const observer = new MutationObserver(() => {
      handleHeight(tabGroup);
    });
    observer.observe(tabGroup, {
      attributes: true,
      attributeFilter: ["data-cdrm-focused-idx"]
    });
    handleHeight(tabGroup);
    removeInitialRelativePositions(tabGroup);
  }
}
(() => {
  addConundrumTabClickListeners();
  window.addEventListener("resize", handleConundrumTabGroupHeight);
  window.addEventListener("cdrm-content-loaded", addConundrumTabClickListeners);
})();
const handleConundrumTabClick = () => {
  console.log(`Click registered!!`);
};
const onResize = () => {
  handleConundrumTabGroupHeight();
  handleConundrumAdmonitionResize();
};
const onConundrumContentLoaded = () => {
  applyAdmonitionClickListeners();
  applyCopyConundrumCodeBlockListeners();
  addConundrumTabClickListeners();
};
const initializeConundrumStaticWebAssets = () => {
  debugger;
  onResize();
  onConundrumContentLoaded();
  window.addEventListener("cdrm-content-loaded", onConundrumContentLoaded);
  window.addEventListener("resize", onResize);
  window.addEventListener("cdrm-manual-resize", onResize);
  console.info("Initialized Conundrum glue code...");
};
const cleanupConundrumStaticWebAssets = () => {
  window.removeEventListener("cdrm-content-loaded", onConundrumContentLoaded);
  window.removeEventListener("resize", onResize);
  window.removeEventListener("cdrm-manual-resize", onResize);
};
export {
  cleanupConundrumStaticWebAssets,
  handleConundrumTabClick,
  initializeConundrumStaticWebAssets,
  onConundrumContentLoaded
};
//# sourceMappingURL=main.es.js.map
