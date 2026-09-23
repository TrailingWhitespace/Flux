"use client";

import {
  useTodos,
  useCreateTodo,
  useToggleTodo,
  useDeleteTodo,
} from "@/lib/hooks/use-todos";

export default function TodosPage() {
  const { data: todos, isLoading } = useTodos();
  const { mutate: createTodo } = useCreateTodo();
  const { mutate: toggleTodo } = useToggleTodo();
  const { mutate: deleteTodo } = useDeleteTodo();
  // each hook gives us the mutate function and here we rename/alias it to use

  if (isLoading) return <p>Loading...</p>;

  return (
    <main>
      <h1>Todos</h1>
      <ul>
        {todos?.map((todo) => (
          <li key={todo.id}>
            <span onClick={() => toggleTodo(todo.id)}>{todo.todo}</span>
            <button onClick={() => deleteTodo(todo.id)}>Delete</button>
          </li>
        ))}
        <button onClick={() => createTodo("test lol")}>Add test todo</button>
      </ul>
    </main>
  );
}
