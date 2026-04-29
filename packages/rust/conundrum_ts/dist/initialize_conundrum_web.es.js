const o = () => {
  console.log("Tab clicked!!!");
}, n = () => {
  console.log("Copy code here!!!");
}, c = () => {
  console.log("Toggle copy icon here!!!");
}, e = () => {
  console.log("Admonition title clicked!");
}, l = () => {
  window.conundrum = {
    handleConundrumTabClick: o,
    onCodeBlockContainerClick: c,
    onCopyCodeBlockClick: n,
    onAdmonitionHeadingClick: e
  };
};
export {
  l as initializeConundrumWeb
};
//# sourceMappingURL=initialize_conundrum_web.es.js.map
