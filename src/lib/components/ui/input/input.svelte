<script lang="ts">
  import type { Component } from "svelte";
  import type {
    HTMLInputAttributes,
    HTMLInputTypeAttribute,
  } from "svelte/elements";

  import { type WithElementRef, cn } from "$lib/utils.js";

  type InputType = Exclude<HTMLInputTypeAttribute, "file">;

  type Props = WithElementRef<
    Omit<HTMLInputAttributes, "type"> &
      (
        | { type: "file"; files?: FileList }
        | { type?: InputType; files?: undefined }
      ) & {
        startIcon?: Component;
      }
  >;

  let {
    ref = $bindable(null),
    value = $bindable(),
    type,
    files = $bindable(),
    class: className,
    startIcon,
    ...restProps
  }: Props = $props();
</script>

{#if type === "file"}
  <input
    bind:this={ref}
    data-slot="input"
    class={cn(
      "flex h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-2 text-sm font-medium shadow-xs ring-offset-background transition-[color,box-shadow] outline-none selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 md:text-sm dark:bg-input/30",
      "focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
      "aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40",
      className,
    )}
    type="file"
    bind:files
    bind:value
    {...restProps}
  />
{:else}
  <div
    class={cn(
      "flex h-9 w-full min-w-0 items-center rounded-md border border-input bg-background px-3 py-1 text-base shadow-xs ring-offset-background transition-[color,box-shadow] has-disabled:cursor-not-allowed has-disabled:opacity-50 md:text-sm dark:bg-input/30",
      "focus-within:border-ring focus-within:ring-[3px] focus-within:ring-ring/50",
      "has-aria-invalid:border-destructive has-aria-invalid:ring-destructive/20 dark:has-aria-invalid:ring-destructive/40",
      "[&_svg]:mr-2.5 [&_svg]:size-4 [&_svg]:shrink-0",
      className,
    )}
  >
    {#if startIcon}
      {@const Icon = startIcon}
      <Icon />
    {/if}
    <input
      bind:this={ref}
      data-slot="input"
      class={cn(
        "flex-1 outline-none selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground",
      )}
      {type}
      bind:value
      {...restProps}
    />
  </div>
{/if}
