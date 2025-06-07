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
  data-slot="dialog-footer"
  class={cn(
    "-mx-3.5 flex flex-col-reverse gap-2 border-t px-3.5 py-3.5 sm:flex-row sm:justify-end",
    className,
  )}
  {...restProps}
>
  {@render children?.()}
</div>
