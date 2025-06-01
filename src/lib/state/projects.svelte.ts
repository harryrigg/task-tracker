import { invoke } from "@tauri-apps/api/core";

import type { Project } from "$lib/types";

import { tasks } from "./tasks.svelte";

class State {
  private _state = $state<Project[]>([]);

  constructor() {
    (async () => {
      this.reload();
    })();
  }

  async reload() {
    this._state = await invoke("list_projects");
  }

  get projects() {
    return this._state;
  }

  async create(name: string, description: string) {
    const project = await invoke<Project>("create_project", {
      name,
      description,
    });
    await this.reload();
    return project;
  }

  async update(id: number, name: string, description: string) {
    await invoke("update_project", {
      id,
      name,
      description,
    });
    tasks.reload();
    await this.reload();
  }

  async delete(id: number) {
    await invoke("delete_project", {
      id,
    });
    tasks.reload();
    await this.reload();
  }
}

export const projects = new State();
