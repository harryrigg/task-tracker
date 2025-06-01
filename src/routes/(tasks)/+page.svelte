<script lang="ts">
  import { Temporal } from "@js-temporal/polyfill";
  import {
    ArrowRightIcon,
    CalendarIcon,
    PencilIcon,
    TrashIcon,
  } from "@lucide/svelte";
  import { createColumnHelper } from "@tanstack/table-core";

  import EditTaskDialog from "$lib/components/edit-task-dialog.svelte";
  import { buttonVariants } from "$lib/components/ui/button";
  import { Button } from "$lib/components/ui/button";
  import { Calendar } from "$lib/components/ui/calendar";
  import { Card } from "$lib/components/ui/card";
  import { DataTable, renderSnippet } from "$lib/components/ui/data-table";
  import * as Popover from "$lib/components/ui/popover";
  import { TooltipSimple } from "$lib/components/ui/tooltip";

  import { tasks } from "$lib/state/tasks.svelte";
  import { type Task } from "$lib/types";
  import { IntlDateConvert, cn, formatDuration, formatTime } from "$lib/utils";

  import TaskCard from "./task-card.svelte";

  const columnHelper = createColumnHelper<Task>();

  const columns = [
    columnHelper.accessor("project.name", {
      header: "Project",
      cell: ({ row }) => renderSnippet(projectColumn, { task: row.original }),
    }),
    columnHelper.accessor("description", {
      header: "Description",
      meta: {
        class:
          "w-full max-w-0 text-muted-foreground text-ellipsis overflow-hidden",
      },
      cell: ({ row }) =>
        renderSnippet(descriptionColumn, { task: row.original }),
    }),
    columnHelper.accessor("startedAt", {
      header: "Started",
      cell: (v) => formatTime(v.getValue()),
    }),
    columnHelper.accessor("finishedAt", {
      header: "Finished",
      cell: (v) => {
        const val = v.getValue();
        return val ? formatTime(val) : "-";
      },
    }),
    columnHelper.accessor(
      (v) => (v.finishedAt ? v.finishedAt.since(v.startedAt) : null),
      {
        id: "duration",
        header: "Duration",
        cell: (v) => {
          const val = v.getValue();
          return val && formatDuration(val, false);
        },
      },
    ),
    columnHelper.display({
      id: "controls",
      cell: ({ row }) => renderSnippet(tableControls, { task: row.original }),
    }),
  ];

  const todayDate = Temporal.Now.plainDateISO();

  let editingTask = $state<Task | null>(null);
</script>

{#snippet projectColumn({ task }: { task: Task })}
  <TooltipSimple tooltip={task.project.description}>
    {task.project.name}
  </TooltipSimple>
{/snippet}

{#snippet descriptionColumn({ task }: { task: Task })}
  {#if task.description === ""}
    <span class="text-muted-foreground/40 italic">N/A</span>
  {:else}
    {task.description}
  {/if}
{/snippet}

{#snippet tableControls({ task }: { task: Task })}
  <span class="flex gap-1">
    <Button
      size="icon"
      variant="ghost"
      icon={TrashIcon}
      onclick={async () => {
        await tasks.delete(task.id);
      }}
    />
    <Button
      size="icon"
      variant="ghost"
      icon={PencilIcon}
      onclick={() => {
        editingTask = task;
      }}
    />
  </span>
{/snippet}

{#if editingTask}
  <EditTaskDialog task={editingTask} onClose={() => (editingTask = null)} />
{/if}

<div class="flex h-full flex-col gap-2">
  <TaskCard />

  <Card class="flex flex-1 flex-col gap-2 overflow-hidden">
    <div class="flex justify-between">
      <Popover.Root>
        <Popover.Trigger
          class={cn(buttonVariants({ variant: "outline", class: "w-[150px]" }))}
        >
          <CalendarIcon />
          {Temporal.PlainDate.compare(tasks.date, todayDate) === 0
            ? "Today"
            : tasks.date.toLocaleString()}
        </Popover.Trigger>
        <Popover.Content class="w-auto p-0" align="start">
          <Calendar
            bind:value={
              () => IntlDateConvert.dateFromTemporal(tasks.date),
              (v) => (tasks.date = IntlDateConvert.dateToTemporal(v))
            }
            type="single"
            preventDeselect
            isDateDisabled={(v) =>
              Temporal.PlainDate.compare(
                IntlDateConvert.dateToTemporal(v),
                todayDate,
              ) > 0}
          />
        </Popover.Content>
      </Popover.Root>
      <Button variant="outline" icon={ArrowRightIcon} iconRight>
        Go to Summary
      </Button>
    </div>
    <DataTable {columns} data={tasks.onDate} />
  </Card>
</div>
