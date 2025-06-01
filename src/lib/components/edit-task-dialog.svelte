<script lang="ts">
  import { useId } from "bits-ui";
  import toast from "svelte-french-toast";
  import { superForm } from "sveltekit-superforms";
  import { zod } from "sveltekit-superforms/adapters";

  import ProjectChooser from "$lib/components/project-chooser.svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Form from "$lib/components/ui/form";
  import { TimeInput } from "$lib/components/ui/time-input";

  import { type TaskSchema, taskSchema } from "$lib/schema";
  import { projects } from "$lib/state/projects.svelte";
  import { tasks } from "$lib/state/tasks.svelte";
  import type { Task } from "$lib/types";

  type Props = {
    task: Task;
    onClose?: () => void;
  };

  let { task, onClose }: Props = $props();

  const initialData = {
    startedAt: task.startedAt.toPlainTime(),
    finishedAt: task.finishedAt!.toPlainTime(),
    projectId: task.project.id,
  } satisfies TaskSchema;

  const form = superForm(initialData, {
    dataType: "json",
    SPA: true,
    validators: zod(taskSchema),
    onUpdate: async ({ form }) => {
      if (!form.valid) return;

      const startedAt = task.startedAt.withPlainTime(form.data.startedAt);
      const finishedAt = task.finishedAt!.withPlainTime(form.data.finishedAt);

      await tasks.update(task.id, form.data.projectId, startedAt, finishedAt);
      toast.success("Updated task");
      onClose?.();
    },
  });
  const { form: formData, enhance, submitting } = form;

  const formId = useId();

  let dialogCloseBehaviour = $derived(
    $submitting ? ("ignore" as const) : ("close" as const),
  );

  const project = $derived(
    projects.projects.find((v) => v.id === $formData.projectId),
  );
</script>

<Dialog.Root
  open={true}
  onOpenChange={(v) => {
    if (!v) {
      onClose?.();
    }
  }}
>
  <Dialog.Content
    closeButton={false}
    interactOutsideBehavior={dialogCloseBehaviour}
    escapeKeydownBehavior={dialogCloseBehaviour}
  >
    <Dialog.Header>
      <Dialog.Title>Edit Task</Dialog.Title>
    </Dialog.Header>
    <form use:enhance id={formId}>
      <Form.Container>
        <Form.Field {form} name="startedAt">
          <Form.Control>
            {#snippet children({ props })}
              <TimeInput
                {...props}
                label="Started At"
                bind:value={$formData.startedAt}
              />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>

        <Form.Field {form} name="finishedAt">
          <Form.Control>
            {#snippet children({ props })}
              <TimeInput
                {...props}
                label="Finished At"
                bind:value={$formData.finishedAt}
              />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>

        <Form.Field {form} name="projectId">
          <Form.Control>
            {#snippet children()}
              <Form.Label>Project</Form.Label>
              <ProjectChooser
                bind:value={
                  () => project || null,
                  (v) => {
                    if (v !== null) {
                      $formData.projectId = v.id;
                    }
                  }
                }
                allowCreate={false}
              />
            {/snippet}
          </Form.Control>
          <Form.FieldErrors />
        </Form.Field>
      </Form.Container>
    </form>
    <Dialog.Footer>
      <Dialog.Close>
        <Button variant="outline" disabled={$submitting}>Cancel</Button>
      </Dialog.Close>
      <Button type="submit" form={formId} disabled={$submitting}>Update</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
