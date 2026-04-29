const handleConundrumTableOfContentsSetup = () => {
  function handleTocHeadingClick(e) {
    const target = e.currentTarget;
    const heading = document.getElementById(`h-${target.getAttribute("cdrm-heading-id")}`);
    if (heading) {
      heading.scrollIntoView({
        behavior: "smooth"
      });
    }
  }
  const items = document.getElementsByClassName("cdrm-toc-item");
  for (var i = 0; i < items.length; i++) {
    const item = items.item(i);
    item?.addEventListener("click", handleTocHeadingClick);
  }
};
(() => {
  handleConundrumTableOfContentsSetup();
  window.addEventListener("cdrm-content-loaded", handleConundrumTableOfContentsSetup);
})();
//# sourceMappingURL=toc.es.js.map
