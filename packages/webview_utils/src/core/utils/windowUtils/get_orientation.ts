import { sendToSwift, SwiftHandler } from "../bridge/send_to_swift";

export enum DeviceOrientation {
    landscape,
    portrait,
}

export const getDeviceOrientation = (
    orientation: ScreenOrientation,
): DeviceOrientation => {
    return orientation.type.startsWith("landscape")
        ? DeviceOrientation.landscape
        : DeviceOrientation.portrait;
};

export const sendOrientationSwiftEvents = (): void => {
    sendToSwift(
        SwiftHandler.setSplitviewEditorLandscapeView,
        getDeviceOrientation(screen.orientation) === DeviceOrientation.landscape
            ? "true"
            : "false",
    );
};

export const setOrientationListener = (): void => {
    const handleOrientationChange = (orientation: ScreenOrientation) => {
        const isLandscape =
            getDeviceOrientation(orientation) === DeviceOrientation.landscape;
        document
            .getElementById("webview-container")
            ?.classList[isLandscape ? "add" : "remove"]("landscape");
        sendToSwift(
            SwiftHandler.setSplitviewEditorLandscapeView,
            isLandscape ? "true" : "false",
        );
    };

    // set initially
    handleOrientationChange(screen.orientation);

    // add listener
    screen.orientation.addEventListener("change", (e) =>
        handleOrientationChange(e.target as ScreenOrientation),
    );
};
