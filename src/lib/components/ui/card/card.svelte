<script lang="ts">
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";

  type Props = {
    children?: Snippet;
    class?: string;
  };

  const { children, class: className }: Props = $props();
</script>

<div
  class={cn(
    "rounded-xl border bg-card p-4 text-card-foreground shadow",
    className,
  )}
>
  {@render children?.()}
</div>
