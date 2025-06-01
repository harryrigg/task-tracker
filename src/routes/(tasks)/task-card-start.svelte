<script lang="ts">
  import { BriefcaseBusinessIcon, PlayIcon } from "@lucide/svelte";

  import ProjectChooser from "$lib/components/project-chooser.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";

  import { tasks } from "$lib/state/tasks.svelte";
  import type { Project } from "$lib/types";
  import { cn } from "$lib/utils";

  let selectedProject: Project | null = $state(null);
  let taskDescription = $state("");

  async function onStart() {
    if (selectedProject === null) return;

    await tasks.start(selectedProject.id, taskDescription);
  }
</script>

<div class="flex h-full items-center justify-between">
  <div class="flex gap-2">
    <ProjectChooser bind:value={selectedProject} allowCreate={true}>
      {#snippet child({ props })}
        <Button
          {...props}
          variant="outline"
          class={cn(
            "h-10 w-[160px] justify-between gap-1",
            selectedProject === null && "border-dashed text-muted-foreground",
            selectedProject !== null && "dark:border-white/50",
          )}
        >
          {#if selectedProject === null}
            Select project
            <BriefcaseBusinessIcon />
          {:else}
            <BriefcaseBusinessIcon />
            <span class="flex-1 text-center">
              {selectedProject.name}
            </span>
          {/if}
        </Button>
      {/snippet}
    </ProjectChooser>

    <Input
      bind:value={taskDescription}
      placeholder="Enter task description..."
      class="h-10 w-[250px]"
    />
  </div>
  <Button size="icon" disabled={selectedProject === null} onclick={onStart}>
    <PlayIcon />
  </Button>
</div>
