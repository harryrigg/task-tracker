<script lang="ts">
  import type { Time as IntlTime } from "@internationalized/date";
  import { Temporal } from "@js-temporal/polyfill";
  import { Clock3Icon } from "@lucide/svelte";
  import { TimeField } from "bits-ui";

  import { Button } from "$lib/components/ui/button";
  import { LABEL_CLASSNAME } from "$lib/components/ui/label/label.svelte";

  import { IntlDateConvert, cn, roundTimeDown } from "$lib/utils";

  type Props = {
    name: string;
    label: string;
    value: Temporal.PlainTime;
    enableSetNow?: boolean;
    "aria-invalid": "true" | undefined;
  };

  let {
    name,
    label,
    value = $bindable(),
    enableSetNow = false,
    "aria-invalid": invalid,
  }: Props = $props();

  function getValue() {
    return IntlDateConvert.timeFromTemporal(value);
  }

  function setValue(newValue: IntlTime) {
    value = roundTimeDown(IntlDateConvert.timeToTemporal(newValue));
  }
</script>

<TimeField.Root bind:value={getValue, setValue}>
  <div class="flex w-full max-w-[280px] flex-col">
    <TimeField.Label class={LABEL_CLASSNAME}>{label}</TimeField.Label>
    <TimeField.Input
      {name}
      aria-invalid={invalid}
      class={cn(
        "flex h-9 w-full items-center px-2 py-3",
        "rounded-md border text-sm tracking-[0.01em] text-foreground select-none dark:bg-input/30",
        "shadow-xs ring-offset-background transition-[color,box-shadow]",
        "focus-within:border-ring focus-within:ring-[3px] focus-within:ring-ring/50",
        "aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40",
      )}
    >
      {#snippet children({ segments })}
        {#each segments as { part, value }, i (part + i)}
          <div class="inline-block select-none">
            {#if part === "literal"}
              <TimeField.Segment {part} class="p-1 text-muted-foreground">
                {value}
              </TimeField.Segment>
            {:else}
              <TimeField.Segment
                {part}
                class="rounded-[5px] border-primary px-1 py-1 hover:bg-muted focus:bg-muted focus:text-foreground focus-visible:ring-0! focus-visible:ring-offset-0! aria-[valuetext=Empty]:text-muted-foreground data-invalid:text-destructive"
              >
                {value}
              </TimeField.Segment>
            {/if}
          </div>
        {/each}
        {#if enableSetNow}
          <Button
            variant="ghost"
            size="none"
            onclick={() => (value = roundTimeDown(Temporal.Now.plainTimeISO()))}
            class="ml-auto cursor-pointer gap-1 px-0.5 text-xs text-foreground/50"
          >
            <Clock3Icon class="size-3" />
            Now
          </Button>
        {/if}
      {/snippet}
    </TimeField.Input>
  </div>
</TimeField.Root>
