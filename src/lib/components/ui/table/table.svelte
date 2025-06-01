<script lang="ts">
  import type { HTMLTableAttributes } from "svelte/elements";

  import { type WithElementRef, cn } from "$lib/utils.js";

  type Props = {
    wrapperClass?: string;
  } & WithElementRef<HTMLTableAttributes>;

  let {
    ref = $bindable(null),
    class: className,
    wrapperClass,
    children,
    ...restProps
  }: Props = $props();
</script>

<div
  data-slot="table-container"
  class={cn("relative w-full overflow-x-auto", wrapperClass)}
>
  <table
    bind:this={ref}
    data-slot="table"
    class={cn("w-full caption-bottom text-sm", className)}
    {...restProps}
  >
    {@render children?.()}
  </table>
</div>
