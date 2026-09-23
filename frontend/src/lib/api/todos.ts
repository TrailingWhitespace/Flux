// src/lib/api/todos.ts
import { api } from "./client";

export interface Todo {
  id: string;
  title: string;
  description: string | null;
  completed: boolean;
  completedAt: number | null;
  priority: number | null;
  dueDate: number | null;
  createdAt: number;
  deletedAt: number | null;
}

export interface TodoInput {
  title: string;
  description?: string;
  priority?: number;
  dueDate?: number;
}

export const todosApi = {
  list: (completed?: boolean) =>
    api.get<Todo[]>(
      `/todos${completed !== undefined ? `?completed=${completed}` : ""}`,
    ),
  create: (input: TodoInput) => api.post<Todo>("/todos/insert_todo", input),
  remove: (id: string) => api.delete<Todo>(`/todos/${id}/delete_todo`),
  restore: (id: string) => api.post<Todo>(`/todos/${id}/restore_todo`),
  toggle: (id: string) => api.post<Todo>(`/todos/${id}/toggle`),
  update: (id: string, input: TodoInput) =>
    api.put<Todo>(`/todos/${id}/update_todo`, input),
};