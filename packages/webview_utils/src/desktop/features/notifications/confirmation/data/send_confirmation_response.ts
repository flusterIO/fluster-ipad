
export const sendConfirmationResponse = (
    wasAccepted: boolean,
    confirmationId: string
) => {
    window.dispatchEvent(
        new CustomEvent("confirmation-response", {
            detail: {
                response: wasAccepted,
                id: confirmationId,
            },
        })
    );
};

export { };
