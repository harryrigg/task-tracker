<script lang="ts">
  import type { Snippet } from "svelte";

  import * as Tooltip from "$lib/components/ui/tooltip";

  import { cn } from "$lib/utils";

  type Props = {
    tooltip: string;
    class?: string;
    children: Snippet;
  };

  let { tooltip, class: className, children }: Props = $props();
</script>

<Tooltip.Provider>
  <Tooltip.Root delayDuration={100}>
    <Tooltip.Trigger class={cn("w-fit text-start", className)}
      >{@render children()}</Tooltip.Trigger
    >
    <Tooltip.Content side="right">{tooltip}</Tooltip.Content>
  </Tooltip.Root>
</Tooltip.Provider>
