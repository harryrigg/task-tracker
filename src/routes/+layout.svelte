<script lang="ts">
  import { page } from "$app/state";
  import { BriefcaseBusinessIcon, ClockIcon } from "@lucide/svelte";
  import type { Snippet } from "svelte";
  import { Toaster } from "svelte-french-toast";

  import "../app.css";

  type Props = {
    children: Snippet;
  };

  let tabs = [
    {
      title: "Tracker",
      url: "/",
      icon: ClockIcon,
    },
    {
      title: "Projects",
      url: "/projects",
      icon: BriefcaseBusinessIcon,
    },
  ];

  let { children }: Props = $props();
</script>

<div class="flex gap-1 border-b bg-muted/50 px-3 pt-2">
  {#each tabs as tab (tab.url)}
    {@const isActive = page.url.pathname.endsWith(tab.url)}
    <a
      href={tab.url}
      data-selected={isActive || undefined}
      class="
        tab relative
        flex w-32 items-center justify-center
        gap-2 rounded-t-md border-x border-t px-1.5
        py-2
        text-sm
        not-data-selected:bg-muted/40
        data-selected:bg-background
        [&_svg]:size-3.5
      "
    >
      <tab.icon />
      {tab.title}
    </a>
  {/each}
</div>
<main class="min-h-0 w-full min-w-0 flex-1 p-3">
  {@render children()}
</main>

<Toaster />
