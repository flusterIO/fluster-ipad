import { WebviewJavascriptError } from "@/code_gen/flat_buffer/shared-webview-data";
import { SharedWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { Builder } from "flatbuffers"
import { uint8ArrayToBase64String } from "./uint8ToBase64";

export const webviewOnError: OnErrorEventHandler = (msg, url, line, column, error) => {
    let builder = new Builder(1024);
    WebviewJavascriptError.startWebviewJavascriptError(builder);
    WebviewJavascriptError.addMessage(builder, builder.createString(typeof msg == "string" ? msg : JSON.stringify(msg)))
    WebviewJavascriptError.addUrl(builder, builder.createString(url));
    WebviewJavascriptError.addLine(builder, builder.createString(`${line}`))
    WebviewJavascriptError.addColumn(builder, builder.createString(`${column}`));
    WebviewJavascriptError.addColumn(builder, builder.createString(JSON.stringify(error)))

    let err = WebviewJavascriptError.endWebviewJavascriptError(builder);

    builder.finish(err);

    let buf = builder.asUint8Array()

    if (window.webkit) {
        window.webkit.messageHandlers[SharedWebviewActions.JavascriptError].postMessage(uint8ArrayToBase64String(buf));
    } else {
        console.log("Error:", msg);
    }
}
