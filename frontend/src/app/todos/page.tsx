import { todosApi } from "@/lib/api/todos";

export default async function TodosPage() {
  let todos = await todosApi.list();

  return (
    <main>
      <h1>this is todos yes</h1>
      <ul>
        {todos.map((todo) => (
          <li key={todo.id}>{todo.id}. {todo.todo}</li>
        ))}
      </ul>
    </main>
  );
}