import type { Temporal } from "@js-temporal/polyfill";

export type Project = {
  id: number;
  name: string;
  description: string;
};

export type TaskDTO = {
  id: number;
  startedAt: string;
  finishedAt: string | null;
  description: string;
  project: Project;
};

export type Task = {
  id: number;
  startedAt: Temporal.ZonedDateTime;
  finishedAt: Temporal.ZonedDateTime | null;
  description: string;
  project: Project;
};
