"use client";
import React, { type ReactNode, useEffect, useEffectEvent, useState } from "react";
import { sendConfirmationResponse } from "../../data/send_confirmation_response";
import { motion } from "framer-motion";
import { Button } from "@/shared_components/shad/button";
import { H4 } from "@/shared_components/typography/typography";
import ModalBackdrop from "src/desktop/core/ui_utils/modal_backdrop";
import { ConfirmationModalProps } from "../../confirmation_types";

const ConfirmationModal = (
    props: ConfirmationModalProps & { handleClose: () => void }
): ReactNode => {
    const [open, setOpen] = useState(false);

    const handleOpen = useEffectEvent(() => setOpen(true))
    useEffect(() => {
        /// Produces the transition while keeping this out of the dom when not in use.
        handleOpen()
    }, []);
    const closeModal = (): void => {
        setOpen(false);
        if (props.handleClose) {
            props.handleClose();
        }
    };
    return (
        <ModalBackdrop onClick={closeModal}>
            <motion.div
                className="max-w-[min(600px,90vw)] border rounded p-4 flex flex-col justify-center items-start gap-6 bg-popover text-foreground"
                onClick={(e) => {
                    e.stopPropagation();
                }}
                variants={{
                    closed: {
                        y: -200,
                        opacity: 0,
                    },
                    opened: {
                        y: 0,
                        opacity: 1,
                    },
                }}
                animate={open ? "opened" : "closed"}
            >
                <div>
                    <H4>{props.title}</H4>
                    <div>{props.body}</div>
                </div>
                <div className="w-full flex flex-row justify-end items-center gap-6">
                    {props.denyButtonText && (
                        <Button
                            variant={"secondary"}
                            onClick={() => {
                                sendConfirmationResponse(false, props.id);
                                closeModal();
                            }}
                        >
                            {props.denyButtonText}
                        </Button>
                    )}
                    {props.acceptButtonText && (
                        <Button
                            onClick={() => {
                                sendConfirmationResponse(true, props.id);
                                closeModal();
                            }}
                            variant={props.confirmationVariant}
                        >
                            {props.acceptButtonText}
                        </Button>
                    )}
                </div>
            </motion.div>
        </ModalBackdrop>
    );
};

ConfirmationModal.displayName = "ConfirmationModal";

export default ConfirmationModal;
