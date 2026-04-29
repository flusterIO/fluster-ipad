"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ConundrumEvents = void 0;
var ConundrumEvents;
(function (ConundrumEvents) {
    /// Useful for things like resizable pains, to more easily emit your own resize event that Conundrum will handle to update the components accordingly.
    ConundrumEvents["ManualResize"] = "cdrm-manual-resize";
})(ConundrumEvents || (exports.ConundrumEvents = ConundrumEvents = {}));
