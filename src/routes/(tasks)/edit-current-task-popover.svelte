<script lang="ts">
  import { PencilIcon } from "@lucide/svelte";
  import toast from "svelte-french-toast";
  import { superForm } from "sveltekit-superforms";
  import { zod } from "sveltekit-superforms/adapters";

  import { Button } from "$lib/components/ui/button";
  import * as Form from "$lib/components/ui/form";
  import { Input } from "$lib/components/ui/input";
  import * as Popover from "$lib/components/ui/popover";
  import { TimeInput } from "$lib/components/ui/time-input";

  import { type CurrentTaskSchema, currentTaskSchema } from "$lib/schema";
  import { tasks } from "$lib/state/tasks.svelte";
  import type { Task } from "$lib/types";

  type Props = {
    task: Task;
  };

  let { task }: Props = $props();

  function getInitialData() {
    return {
      description: task.description,
      startedAt: task.startedAt.toPlainTime(),
      projectId: task.project.id,
    } satisfies CurrentTaskSchema;
  }

  const form = superForm(getInitialData(), {
    dataType: "json",
    SPA: true,
    resetForm: () => {
      form.form.set(getInitialData());
      return false;
    },
    validators: zod(currentTaskSchema),
    onUpdate: async ({ form }) => {
      if (!form.valid) return;

      const startedAt = task.startedAt.withPlainTime(form.data.startedAt);

      await tasks.update(
        task.id,
        form.data.projectId,
        form.data.description,
        startedAt,
        null,
      );
      toast.success("Updated task");
      setOpen(false);
    },
  });
  const { form: formData, enhance } = form;

  let open = $state(false);
  function getOpen() {
    return open;
  }
  function setOpen(value: boolean) {
    open = value;
    if (!open) {
      form.reset();
    }
  }
</script>

<Popover.Root bind:open={getOpen, setOpen}>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button {...props} variant="ghost" size="icon">
        <PencilIcon />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content
    onOpenAutoFocus={(e) => e.preventDefault()}
    align="end"
    class="mt-2"
  >
    <form use:enhance class="flex flex-col gap-2">
      <Form.Container>
        <Form.Field {form} name="description">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label>Description</Form.Label>
              <Input {...props} bind:value={$formData.description} />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>

        <Form.Field {form} name="startedAt">
          <Form.Control>
            {#snippet children({ props })}
              <TimeInput
                {...props}
                label="Started At"
                enableSetNow
                bind:value={$formData.startedAt}
              />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>
      </Form.Container>
      <Button type="submit">Save</Button>
    </form>
  </Popover.Content>
</Popover.Root>
