import { Temporal } from "@js-temporal/polyfill";
import { invoke } from "@tauri-apps/api/core";

import { type Task, type TaskDTO } from "$lib/types";
import { createDateTime } from "$lib/utils";

function transformTask(v: TaskDTO): Task {
  return {
    ...v,
    startedAt: createDateTime(v.startedAt),
    finishedAt: v.finishedAt ? createDateTime(v.finishedAt) : null,
  };
}

class TasksState {
  _current = $state<Task | null>(null);

  _date = $state<Temporal.PlainDate>(Temporal.Now.plainDateISO());
  _onDate = $state<Task[]>([]);

  constructor() {
    (async () => {
      await this.reload();
    })();
  }

  async reload() {
    try {
      this._current = await invoke<TaskDTO>("current_task").then(transformTask);
    } catch (e) {
      if (e === "no value found") {
        this._current = null;
      } else {
        throw e;
      }
    }

    const startTime = this._date.toZonedDateTime({
      timeZone: Temporal.Now.timeZoneId(),
    });
    const endTime = startTime.add({ days: 1 });
    const utcStart = startTime.toInstant().toString();
    const utcEnd = endTime.toInstant().toString();

    this._onDate = await invoke<TaskDTO[]>("list_tasks_between", {
      start: utcStart,
      end: utcEnd,
    }).then((v) => v.map(transformTask));
  }

  async start(projectId: number, description: string) {
    await invoke("start_task", { projectId, description });
    await this.reload();
  }

  async stop() {
    await invoke("stop_task");
    await this.reload();
  }

  async update(
    id: number,
    projectId: number,
    description: string,
    startedAt: Temporal.ZonedDateTime,
    finishedAt: Temporal.ZonedDateTime | null,
  ) {
    const startedAtUtc = startedAt.toInstant().toString();
    const finishedAtUtc = finishedAt?.toInstant().toString();

    await invoke("update_task", {
      id,
      projectId,
      description,
      startedAt: startedAtUtc,
      finishedAt: finishedAtUtc,
    });
    await this.reload();
  }

  async delete(id: number) {
    await invoke("delete_task", {
      id,
    });
    await this.reload();
  }

  get current() {
    return this._current;
  }

  get date() {
    return this._date;
  }

  set date(value) {
    this._date = value;
    this.reload();
  }

  get onDate() {
    return this._onDate;
  }
}

export const tasks = new TasksState();
