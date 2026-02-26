import { UserFacingNotification, UserFacingNotificationVariant } from "@/code_gen/flat_buffer/notifications";
import { MdxPreviewWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { Builder } from "flatbuffers";


interface GetUserFacingNotificationProps {
    title: string;
    body: string;
    variant?: UserFacingNotificationVariant
}

export const getUserFacingNotification = ({ title, body, variant }: GetUserFacingNotificationProps): Uint8Array => {
    const builder = new Builder(1024)
    const titleOffset = builder.createString(title)
    const bodyOffset = builder.createString(body)
    UserFacingNotification.startUserFacingNotification(builder)
    UserFacingNotification.addTitle(builder, titleOffset)
    UserFacingNotification.addBody(builder, bodyOffset)
    UserFacingNotification.addVariant(builder, variant ?? UserFacingNotificationVariant.Toast)
    const userFacingNotificationOffset = UserFacingNotification.endUserFacingNotification(builder)
    builder.finish(userFacingNotificationOffset)
    return builder.asUint8Array()
}



export const sendUserFacingNotification = (props: GetUserFacingNotificationProps) => {
    return sendToSwift(MdxPreviewWebviewActions.ShowUserFacingNotification, getUserFacingNotification(props).toString())
}
