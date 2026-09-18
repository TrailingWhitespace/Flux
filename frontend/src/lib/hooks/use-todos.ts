// src/lib/hooks/use-todos.ts
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { todosApi } from "@/lib/api/todos";

export function useTodos(completed?: boolean) {
  return useQuery({
    queryKey: ["todos", completed], 
    // completed is for if i call useTodos(true) and get only completed todos,
    // it doesnt overwrite the todos already in cache from like if i call useTodos() (without true)
    // invalidating using the query key ["todos"], also invalidates the completed list aswell because it doesnt check
    // array equality but the array prefix 
    // so both the lists, default "all" todos and only completed todos, get marked stale and get invalidated and refetched
    // when a mutation like create or toggle or delete does queryClient.invalidateQueries({ queryKey: ["todos"] })
    queryFn: () => todosApi.list(completed),
  });
}

export function useCreateTodo() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (todo: string) => todosApi.create(todo),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["todos"] }),
  });
}

export function useToggleTodo() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => todosApi.toggle(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["todos"] }),
  });
}

export function useUpdateTodo() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, todo }: { id: number; todo: string }) => todosApi.update(id, todo),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["todos"] }),
  });
}

export function useDeleteTodo() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => todosApi.remove(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["todos"] }),
  });
}