<script lang="ts">
  import { PencilIcon, PlusIcon, SearchIcon, TrashIcon } from "@lucide/svelte";
  import { createColumnHelper } from "@tanstack/table-core";

  import CreateProjectDialog from "$lib/components/create-project-dialog.svelte";
  import EditProjectDialog from "$lib/components/edit-project-dialog.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Card } from "$lib/components/ui/card";
  import { DataTable, renderSnippet } from "$lib/components/ui/data-table";
  import { Input } from "$lib/components/ui/input";

  import { projects } from "$lib/state/projects.svelte";
  import type { Project } from "$lib/types";

  const columnHelper = createColumnHelper<Project>();

  const columns = [
    columnHelper.accessor("name", {
      header: "Name",
    }),
    columnHelper.accessor("description", {
      header: "Description",
      meta: {
        class: "w-full",
      },
    }),
    columnHelper.display({
      id: "controls",
      cell: ({ row }) =>
        renderSnippet(tableControls, { project: row.original }),
    }),
  ];

  let editingProject = $state<Project | null>();
  let searchValue = $state("");
</script>

{#snippet tableControls({ project }: { project: Project })}
  <span class="flex gap-1">
    <Button
      size="icon"
      variant="ghost"
      icon={TrashIcon}
      onclick={() => {
        projects.delete(project.id);
      }}
    />
    <Button
      size="icon"
      variant="ghost"
      icon={PencilIcon}
      onclick={() => {
        editingProject = project;
      }}
    />
  </span>
{/snippet}

{#if editingProject}
  <EditProjectDialog
    project={editingProject}
    onClose={() => (editingProject = null)}
  />
{/if}

<Card class="flex h-full flex-col gap-2 overflow-hidden">
  <div class="flex justify-between">
    <Input
      bind:value={searchValue}
      startIcon={SearchIcon}
      placeholder="Search"
      class="max-w-72"
    />
    <CreateProjectDialog>
      {#snippet trigger({ props })}
        <Button icon={PlusIcon} size="sm" class="h-auto" {...props}>
          Add Project
        </Button>
      {/snippet}
    </CreateProjectDialog>
  </div>

  <DataTable {columns} data={projects.projects} globalFilter={searchValue} />
</Card>
