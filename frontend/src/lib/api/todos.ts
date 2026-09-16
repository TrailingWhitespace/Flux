import { api } from "./client";

export interface Todo {
  id: number;
  todo: string;
  completed: boolean;
  completedAt: number | null;
}

export const todosApi = {
  list: (completed?: boolean) =>
    api.get<Todo[]>(`/todos${completed !== undefined ? `?completed=${completed}` : ""}`),
  create: (todo: string) => api.post<Todo>("/todos/insert_todo", { todo }),
  remove: (id: number) => api.delete<Todo>(`/todos/${id}/delete_todo`), 
  toggle: (id: number) => api.post<Todo>(`/todos/${id}/toggle`),
  update: (id: number, todo: string) => api.put<Todo>(`/todos/${id}/update_todo`, { todo }),
};