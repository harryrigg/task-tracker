<script lang="ts">
  import { Temporal } from "@js-temporal/polyfill";
  import { onMount } from "svelte";

  import { Button } from "$lib/components/ui/button";
  import { TooltipSimple } from "$lib/components/ui/tooltip";

  import { tasks } from "$lib/state/tasks.svelte";
  import type { Task } from "$lib/types";
  import { formatDuration, formatTime } from "$lib/utils";

  import EditCurrentTaskPopover from "./edit-current-task-popover.svelte";

  type Props = {
    currentTask: Task;
  };

  let { currentTask }: Props = $props();

  const getTimer = () =>
    Temporal.Now.zonedDateTimeISO().since(currentTask.startedAt);

  let timer = $state(getTimer());

  onMount(() => {
    const intervalId = setInterval(() => (timer = getTimer()), 100);
    return () => clearInterval(intervalId);
  });

  async function onStop() {
    await tasks.stop();
  }
</script>

{#snippet detail(label: string, value: string)}
  <span class="mr-2 flex flex-col">
    <span class="text-xs text-muted-foreground/80">{label}</span>
    <span class="text-lg/5 font-medium text-card-foreground/80">{value}</span>
  </span>
{/snippet}

<div class="flex h-full items-center justify-between">
  <div class="flex flex-col">
    <TooltipSimple tooltip={currentTask.project.description} class="text-lg/6">
      {currentTask.project.name}
    </TooltipSimple>
    <span class="text-sm text-muted-foreground/90">
      {#if currentTask.description === ""}
        <i>No task description...</i>
      {:else}
        {currentTask.description}
      {/if}
    </span>
  </div>
  <div class="flex items-center gap-3">
    {@render detail("Started At", formatTime(currentTask.startedAt))}
    {@render detail("Elapsed", formatDuration(timer))}
    <div class="flex items-center gap-1.5">
      <EditCurrentTaskPopover task={currentTask} />
      <Button size="icon" onclick={() => onStop()}>
        <div class="size-3.5 rounded-xs bg-foreground"></div>
      </Button>
    </div>
  </div>
</div>
