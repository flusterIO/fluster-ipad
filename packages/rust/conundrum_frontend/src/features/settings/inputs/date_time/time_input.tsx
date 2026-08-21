import React, { useEffect, useState, type ReactNode } from "react";
import {
    type Path,
    type FieldValues,
    useFormContext,
} from "react-hook-form";
import { ChevronDownIcon } from "lucide-react";
import dayjs, { type Dayjs } from "dayjs";
import { Button } from "@/components/shad/button";
import { Calendar } from "@/components/shad/calendar";
import { Input } from "@/components/shad/input";
import { Label } from "@/components/shad/label";
import {
    PopoverContent,
    PopoverTrigger,
    Popover,
} from "@/components/shad/popover";
import { cn } from "@/utils/shad_utils";

interface DateTimeInputProps<T extends FieldValues> {
    name: Path<T>;
    formatter: (d: Dayjs) => T[Path<T>];
    hideDate?: boolean;
    classes?: {
        container?: string;
        dateFormItem?: string;
        calendar?: string;
        calendarPopoverContent?: string;
        calendarPopover?: string;
        calendarButton?: string;
        timeInput?: string;
        timeInputLabel?: string;
        calendarInputLabel?: string;
        calendarInputContainer?: string;
        timeInputContainer?: string;
    };
    timeLabel?: string;
}

export const DateTimeInput = <T extends FieldValues>({
    name,
    hideDate,
    formatter,
    timeLabel,
    classes = {},
}: DateTimeInputProps<T>): ReactNode => {
    const form = useFormContext();
    const formValue = form.watch(name);
    const now = formValue ? (new Date(formValue)) : new Date();
    const [calendarOpen, setCalendarOpen] = useState(false);
    const [date, setDate] = useState<Date | undefined>(
        new Date(`${now.getMonth() + 1}/${now.getDate()}/${now.getFullYear()}`),
    );
    const [time, setTime] = useState<string | undefined>("09:00:00");
    const handleDateTime = (): void => {
        if (!time || !date) {
            return;
        }
        const [hours, minutes, seconds] = time.split(":");
        let d = dayjs(date);
        d = d.add(parseInt(hours), "hours");
        d = d.add(parseInt(minutes), "minutes");
        d = d.add(parseInt(seconds), "seconds");
        form.setValue(name, formatter(d));
    };
    useEffect(() => {
        handleDateTime();
    }, [date, time]);

    return (
        <div className={cn("flex gap-4", classes.container)}>
            {hideDate ? null : (<div
                className={cn("flex flex-col gap-3", classes.calendarInputContainer)}
            >
                <Label
                    htmlFor="date-picker"
                    className={cn("px-1", classes.calendarInputLabel)}
                >
                    Date
                </Label>
                <Popover open={calendarOpen} onOpenChange={setCalendarOpen}>
                    <PopoverTrigger asChild>
                        <Button
                            variant="outline"
                            id="date-picker"
                            className={cn(
                                "w-32 justify-between font-normal",
                                classes.calendarButton,
                            )}
                        >
                            {date ? date.toLocaleDateString() : "Select date"}
                            <ChevronDownIcon />
                        </Button>
                    </PopoverTrigger>
                    <PopoverContent
                        className={cn(
                            "w-auto overflow-hidden p-0",
                            classes.calendarPopover,
                        )}
                        align="start"
                    >
                        <Calendar
                            mode="single"
                            selected={date}
                            captionLayout="dropdown"
                            className={classes.calendar}
                            onSelect={(date) => {
                                setDate(date);
                                /* setOpen(false); */
                            }}
                        />
                    </PopoverContent>
                </Popover>
            </div>)}
            <div className={cn("flex flex-col gap-3", classes.timeInputContainer)}>
                <Label
                    htmlFor="time-picker"
                    className={cn("px-1", classes.timeInputLabel)}
                >
                    {timeLabel ?? "Time"}
                </Label>
                <Input
                    type="time"
                    id="time-picker"
                    step="1"
                    defaultValue={time}
                    className={cn(
                        "bg-background appearance-none [&::-webkit-calendar-picker-indicator]:hidden [&::-webkit-calendar-picker-indicator]:appearance-none",
                        classes.timeInput,
                    )}
                    onChange={(e) => {
                        setTime(e.target.value);
                    }}
                />
            </div>
        </div>
    );
};

DateTimeInput.displayName = "DateTimeInput";
