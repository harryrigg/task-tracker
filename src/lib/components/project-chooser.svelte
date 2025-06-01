<script lang="ts">
  import { ChevronsUpDownIcon, PlusIcon } from "@lucide/svelte";
  import { tick } from "svelte";

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
    triggerClass?: string;
  };

  let { value = $bindable(), allowCreate, triggerClass }: Props = $props();

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
      <Button
        variant="outline"
        {...props}
        class={cn(
          "w-[280px] justify-between border-input bg-input",
          triggerClass,
        )}
        role="combobox"
        aria-expanded={open}
      >
        {value?.name || "Select a project..."}
        <ChevronsUpDownIcon class="ml-2 size-4 shrink-0 opacity-50" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content class="w-[280px] p-0">
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
              class="flex items-center justify-between"
            >
              <span>
                {project.name}
              </span>
              <span class="text-muted-foreground">
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
