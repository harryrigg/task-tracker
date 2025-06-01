<script lang="ts">
  import { PlusIcon } from "@lucide/svelte";
  import { type Snippet, tick } from "svelte";

  import CreateProjectDialog from "$lib/components/create-project-dialog.svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Command from "$lib/components/ui/command";
  import * as Popover from "$lib/components/ui/popover";

  import { projects } from "$lib/state/projects.svelte";
  import type { Project } from "$lib/types";
  import { cn } from "$lib/utils";

  type Props = {
    value: Project | null;
    allowCreate: boolean;
    child: Snippet<[{ props: Record<string, unknown> }]>;
  };

  let {
    value = $bindable(),
    allowCreate,
    child: triggerChild,
  }: Props = $props();

  let open = $state(false);
  let triggerRef = $state<HTMLButtonElement>(null!);

  let createDialogOpen = $state(false);

  function closeAndFocusTrigger() {
    open = false;
    tick().then(() => {
      triggerRef.focus();
    });
  }

  function select(project: Project) {
    value = project;
    closeAndFocusTrigger();
  }
</script>

{#if allowCreate}
  <CreateProjectDialog
    bind:dialogOpen={createDialogOpen}
    onCreate={(project) => select(project)}
  />
{/if}

<Popover.Root bind:open>
  <Popover.Trigger bind:ref={triggerRef}>
    {#snippet child({ props })}
      {@render triggerChild({ props })}
    {/snippet}
  </Popover.Trigger>
  <Popover.Content class="w-[280px] p-0" align="start">
    <Command.Root class="">
      <Command.Input placeholder="Search project..." />
      <Command.List class="flex flex-col overflow-x-hidden overflow-y-hidden">
        <Command.Empty>No projects</Command.Empty>
        <Command.Group
          class="overflow-y-auto"
          style="scrollbar-width: thin; scrollbar-color: var(--color-muted-foreground) transparent !important;"
        >
          {#each projects.projects as project}
            <Command.Item
              onSelect={() => select(project)}
              class={cn(
                "flex items-center justify-between",
                value?.id === project.id && "dark:bg-primary",
              )}
            >
              <span>
                {project.name}
              </span>
              <span class="text-foreground/70">
                {project.description}
              </span>
            </Command.Item>
          {/each}
        </Command.Group>
        {#if allowCreate}
          <Command.Separator />
          <Command.Group class="overflow-visible">
            <Button
              icon={PlusIcon}
              variant="ghost"
              size="sm"
              class="w-full"
              onclick={() => {
                open = false;
                createDialogOpen = true;
              }}
            >
              Add new project
            </Button>
          </Command.Group>
        {/if}
      </Command.List>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
