import { z } from "zod";

export const nullableString = z
    .string()
    .nullable()
    .transform((x) => {
        if (x?.length) {
            return x;
        } else {
            return null;
        }
    });
