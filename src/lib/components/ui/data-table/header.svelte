<script lang="ts" generics="TData">
  import type { Header } from "@tanstack/table-core";

  import FlexRender from "./flex-render.svelte";

  type Props<TData> = {
    header: Header<TData, any>;
  };

  const { header }: Props<TData> = $props();

  const title = $derived(header.column.columnDef.header);
</script>

{#if !header.isPlaceholder}
  {#if typeof title === "string"}
    <span>
      {title}
    </span>
  {:else}
    <FlexRender
      content={header.column.columnDef.header}
      context={header.getContext()}
    />
  {/if}
{/if}
