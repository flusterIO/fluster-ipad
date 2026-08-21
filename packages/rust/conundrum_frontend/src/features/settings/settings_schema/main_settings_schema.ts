import { z } from "zod";

export const defaultExpiresTime = 3 * 3600 * 1000;

export const mainSettingsSchema = z.object({
    name: z.object({
        first: z.string().optional(),
        middle: z.string().optional(),
        last: z.string().optional(),
    }),
    daily_chat: z.object({
        expires_time: z
            .number()
            .default(defaultExpiresTime)
            .describe(
                "The number of milliseconds after midnight in the user's timezone that the daily chat should expire. Defaults to 3am.",
            ),
    }),
});
