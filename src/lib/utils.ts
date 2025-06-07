import {
  type DateValue,
  CalendarDate as IntlDate,
  Time as IntlTime,
} from "@internationalized/date";
import { Temporal } from "@js-temporal/polyfill";
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any }
  ? Omit<T, "children">
  : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & {
  ref?: U | null;
};

export function createDateTime(value: string): Temporal.ZonedDateTime {
  const utcString = value.slice(0, 23) + "Z";
  return Temporal.Instant.from(utcString).toZonedDateTimeISO(
    Temporal.Now.timeZoneId(),
  );
}

export function formatTime(time: Temporal.ZonedDateTime): string {
  return time
    .toPlainTime()
    .toLocaleString(undefined, { hour: "numeric", minute: "2-digit" });
}

export function formatDuration(
  duration: Temporal.Duration,
  seconds: boolean = true,
): string {
  const fmtHours = String(duration.hours).padStart(2, "0");
  const fmtMinutes = String(duration.minutes).padStart(2, "0");
  const fmtSeconds = String(duration.seconds).padStart(2, "0");
  if (seconds) {
    return `${fmtHours}:${fmtMinutes}:${fmtSeconds}`;
  } else {
    return `${fmtHours}:${fmtMinutes}`;
  }
}

export function roundTimeDown(time: Temporal.PlainTime): Temporal.PlainTime {
  return time.round({ smallestUnit: "minute", roundingMode: "floor" });
}

export const IntlDateConvert = {
  timeToTemporal(time: IntlTime): Temporal.PlainTime {
    return Temporal.PlainTime.from(time);
  },
  timeFromTemporal(time: Temporal.PlainTime): IntlTime {
    return new IntlTime(time.hour, time.minute, time.second, time.millisecond);
  },
  dateToTemporal(date: DateValue): Temporal.PlainDate {
    return new Temporal.PlainDate(
      date.year,
      date.month,
      date.day,
      date.calendar.identifier,
    );
  },
  dateFromTemporal(date: Temporal.PlainDate): DateValue {
    return new IntlDate(date.year, date.month, date.day);
  },
};
