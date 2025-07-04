import { Temporal } from "@js-temporal/polyfill";
import z from "zod";

import { projects } from "$lib/state/projects.svelte";

import { roundTimeDown } from "./utils";

const timeSchema = z.custom<Temporal.PlainTime>((val) => {
  return val instanceof Temporal.PlainTime;
});

export const projectSchema = z
  .object({
    _id: z.number().optional(),
    name: z.string().min(3),
    description: z.string(),
  })
  .refine(
    (data) =>
      projects.projects.find(
        (v) => v.id !== data._id && v.name === data.name,
      ) === undefined,
    {
      message: "Project name must be unique",
      path: ["name"],
    },
  );

export type ProjectSchema = z.infer<typeof projectSchema>;

const taskBaseSchema = z.object({
  description: z.string(),
  startedAt: timeSchema,
  projectId: z.number(),
});

export const currentTaskSchema = taskBaseSchema.refine(
  (data) =>
    Temporal.PlainTime.compare(
      roundTimeDown(data.startedAt),
      roundTimeDown(Temporal.Now.plainTimeISO()),
    ) <= 0,
  {
    message: "Start date must be in the past",
    path: ["startedAt"],
  },
);

export type CurrentTaskSchema = z.infer<typeof currentTaskSchema>;

export const taskSchema = taskBaseSchema
  .extend({
    finishedAt: timeSchema,
  })
  .refine(
    (data) => Temporal.PlainTime.compare(data.startedAt, data.finishedAt) <= 0,
    {
      message: "Finish date must be after start date",
      path: ["finishedAt"],
    },
  );

export type TaskSchema = z.infer<typeof taskSchema>;
