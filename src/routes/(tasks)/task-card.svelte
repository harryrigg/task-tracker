<script lang="ts">
  import { Card } from "$lib/components/ui/card";

  import { tasks } from "$lib/state/tasks.svelte";

  import CurrentTask from "./task-card-current.svelte";
  import StartTask from "./task-card-start.svelte";
</script>

<Card class="h-[80px]">
  {#if tasks.current === null}
    <StartTask />
  {:else}
    <CurrentTask currentTask={tasks.current} />
  {/if}
</Card>
