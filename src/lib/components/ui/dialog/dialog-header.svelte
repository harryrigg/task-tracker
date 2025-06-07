<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";

  import { type WithElementRef, cn } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    children,
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
  bind:this={ref}
  data-slot="dialog-header"
  class={cn(
    "-mx-3.5 flex flex-col gap-2 border-b px-3.5 py-3.5 text-center sm:text-left",
    className,
  )}
  {...restProps}
>
  {@render children?.()}
</div>
