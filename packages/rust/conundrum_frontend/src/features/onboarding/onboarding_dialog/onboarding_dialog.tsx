import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "@/components/shad/dialog";
import React, { type ReactNode } from "react";

interface OnboardingDialogProps {
    /**
     * This is what will get saved in the DB to indicate the modal has been shown, so it's important to make sure this doesn't change.
     */
    onboardingDialogKey: string;
    title: ReactNode;
    desc?: ReactNode;
    body: ReactNode;
    footer?: ReactNode;
}

export const OnboardingDialog = ({
    title,
    desc,
    body,
    footer,
}: OnboardingDialogProps): ReactNode => {
    return (
        <Dialog>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>{title}</DialogTitle>
                    {desc ? <DialogDescription>{desc}</DialogDescription> : desc}
                </DialogHeader>
                <div>{body}</div>
                {footer ? <DialogFooter>{footer}</DialogFooter> : null}
            </DialogContent>
        </Dialog>
    );
};

OnboardingDialog.displayName = "OnboardingDialog";
