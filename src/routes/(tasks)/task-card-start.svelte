<script lang="ts">
  import { PlayIcon } from "@lucide/svelte";

  import ProjectChooser from "$lib/components/project-chooser.svelte";
  import { Button } from "$lib/components/ui/button";

  import { tasks } from "$lib/state/tasks.svelte";
  import type { Project } from "$lib/types";

  let selectedProject: Project | null = $state(null);

  async function onStart() {
    if (selectedProject === null) return;

    await tasks.start(selectedProject.id);
  }
</script>

<div class="flex h-full items-center justify-between">
  <ProjectChooser
    bind:value={selectedProject}
    allowCreate={true}
    triggerClass="h-10 border-dashed"
  />
  <Button size="icon" disabled={selectedProject === null} onclick={onStart}>
    <PlayIcon />
  </Button>
</div>
