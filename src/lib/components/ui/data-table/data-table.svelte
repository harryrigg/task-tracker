<script lang="ts" generics="TData">
  import {
    type ColumnDef,
    getCoreRowModel,
    getFilteredRowModel,
  } from "@tanstack/table-core";

  import { FlexRender, createSvelteTable } from "$lib/components/ui/data-table";
  import { ScrollArea } from "$lib/components/ui/scroll-area";
  import * as Table from "$lib/components/ui/table";

  import { cn } from "$lib/utils";

  import Header from "./header.svelte";

  type Props<TData> = {
    columns: ColumnDef<TData, any>[];
    data: TData[];
    globalFilter?: string;
    class?: string;
  };

  const {
    data,
    columns,
    globalFilter,
    class: className,
  }: Props<TData> = $props();

  const table = createSvelteTable({
    get data() {
      return data;
    },
    get columns() {
      return columns;
    },
    state: {
      get globalFilter() {
        return globalFilter ? [globalFilter] : [];
      },
    },
    getCoreRowModel: getCoreRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
  });
</script>

<div class={cn("h-full min-h-0 rounded-md", className)}>
  <ScrollArea class="h-full" scrollbarYClasses="pt-[45px]">
    <Table.Root wrapperClass="overflow-x-visible">
      <Table.Header class="sticky top-0">
        {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
          <Table.Row style="border-bottom: 0">
            {#each headerGroup.headers as header (header.id)}
              <Table.Head
                class="bg-accent px-3 first:rounded-l-md last:rounded-r-md"
              >
                <Header {header} />
              </Table.Head>
            {/each}
          </Table.Row>
        {/each}
      </Table.Header>
      <Table.Body>
        {#each table.getRowModel().rows as row, i (row.id)}
          <Table.Row
            data-state={row.getIsSelected() && "selected"}
            style="border-bottom: 0"
          >
            {#each row.getVisibleCells() as cell (cell.id)}
              <Table.Cell
                class={cn(
                  "px-3 first:rounded-l-md last:rounded-r-md",
                  cell.column.columnDef.meta?.class,
                )}
              >
                <FlexRender
                  content={cell.column.columnDef.cell}
                  context={cell.getContext()}
                />
              </Table.Cell>
            {/each}
          </Table.Row>
        {:else}
          <Table.Row>
            <Table.Cell colspan={columns.length} class="h-24 text-center">
              No results.
            </Table.Cell>
          </Table.Row>
        {/each}
      </Table.Body>
    </Table.Root>
  </ScrollArea>
</div>
