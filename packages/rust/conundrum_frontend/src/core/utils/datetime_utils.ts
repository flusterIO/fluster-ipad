import dayjs from "dayjs"
import advancedFormat from "dayjs/plugin/advancedFormat";
dayjs.extend(advancedFormat);


export const humanReadableDateTime = (date: Date, format: "long" | "short" | "full-with-time", fullWithTimeJoiningWord = "at"): string => {
    return dayjs(date).format({
        short: "MM/DD/YYYY",
        long: "MMM Do, YYYY",
        ["full-with-time"]: `MMM Do, YYYY [${fullWithTimeJoiningWord}] hh:mm a`
    }[format])
}
