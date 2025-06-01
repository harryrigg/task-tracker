<script lang="ts">
  import { useId } from "bits-ui";
  import toast from "svelte-french-toast";
  import { superForm } from "sveltekit-superforms";
  import { zod } from "sveltekit-superforms/adapters";

  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Form from "$lib/components/ui/form";
  import { Input } from "$lib/components/ui/input";

  import { type ProjectSchema, projectSchema } from "$lib/schema";
  import { projects } from "$lib/state/projects.svelte";
  import type { Project } from "$lib/types";

  type Props = {
    project: Project;
    onClose?: () => void;
  };

  let { project, onClose }: Props = $props();

  const initialData = {
    name: project.name,
    description: project.description,
  } satisfies ProjectSchema;

  const form = superForm(initialData, {
    SPA: true,
    validators: zod(projectSchema),
    onUpdate: async ({ form }) => {
      if (!form.valid) return;
      await projects.update(project.id, form.data.name, form.data.description);
      toast.success("Updated project");
      onClose?.();
    },
  });
  const { form: formData, enhance, submitting } = form;

  const formId = useId();

  let dialogCloseBehaviour = $derived(
    $submitting ? ("ignore" as const) : ("close" as const),
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
      <Dialog.Title>Edit Project</Dialog.Title>
    </Dialog.Header>
    <form use:enhance id={formId}>
      <Form.Container>
        <Form.Field {form} name="name">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label>Name</Form.Label>
              <Input {...props} bind:value={$formData.name} />
            {/snippet}
          </Form.Control>
          <Form.Description>Must be unique</Form.Description>
          <Form.FieldErrors />
        </Form.Field>

        <Form.Field {form} name="description">
          <Form.Control>
            {#snippet children({ props })}
              <Form.Label>Description</Form.Label>
              <Input {...props} bind:value={$formData.description} />
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
