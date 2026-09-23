"use client";

import { useRef, useState } from "react";
import {
  useTodos,
  useCreateTodo,
  useToggleTodo,
  useDeleteTodo,
  useRestoreTodo,
  useUpdateTodo,
} from "@/lib/hooks/use-todos";
import type { Todo, TodoInput } from "@/lib/api/todos";
import "./todos.css";
import { TodoModal } from "@/components/Todo-Modal/todo_modal";


type Filter = "all" | "pending" | "completed";
const UNDO_WINDOW_MS = 4000;


// TODO: Modal way to add todos like in daymark 


export default function TodosPage() {
  const { data: todos, isLoading } = useTodos();
  const { mutate: createTodo } = useCreateTodo();
  const { mutate: toggleTodo } = useToggleTodo();
  const { mutate: deleteTodo } = useDeleteTodo();
  const { mutate: restoreTodo } = useRestoreTodo();
  const { mutate: updateTodo } = useUpdateTodo();

  const [input, setInput] = useState("");
  const [filter, setFilter] = useState<Filter>("all");
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editText, setEditText] = useState("");
  const [modalTodoId, setModalTodoId] = useState<string | "new" | null>(null);

  const [pendingDeletes, setPendingDeletes] = useState<Record<string, Todo>>(
    {},
  );
  const timers = useRef<Record<string, ReturnType<typeof setTimeout>>>({});

  if (isLoading) return <p className="todos-loading">Loading…</p>;

  const visible = (todos ?? [])
    .filter((t) => !pendingDeletes[t.id])
    .filter((t) => {
      if (filter === "pending") return !t.completed;
      if (filter === "completed") return t.completed;
      return true;
    });

  const counts = {
    all: (todos ?? []).filter((t) => !pendingDeletes[t.id]).length,
    pending: (todos ?? []).filter((t) => !t.completed && !pendingDeletes[t.id])
      .length,
    completed: (todos ?? []).filter((t) => t.completed && !pendingDeletes[t.id])
      .length,
  };

  


const modalTodo: Todo | null =
  modalTodoId && modalTodoId !== "new"
    ? (todos?.find((t) => t.id === modalTodoId) ?? null)
    : null;

function handleModalSave(input: TodoInput) {
  if (modalTodoId === "new") {
    createTodo(input);
  } else if (modalTodoId) {
    updateTodo({ id: modalTodoId, input });
  }
}

// quick-add stays as-is, just sends the new TodoInput shape:
function handleSubmit(e: React.SubmitEvent) {
  e.preventDefault();
  if (!input.trim()) return;
  createTodo({ title: input.trim() });
  setInput("");
}

  function handleDelete(todo: Todo) {
    // delete immediately, server soft deletes it, just sets deleted_at to a timestamp which we can later set to NULL on undo
    deleteTodo(todo.id);
    setPendingDeletes((prev) => ({ ...prev, [todo.id]: todo }));
    timers.current[todo.id] = setTimeout(() => {
      setPendingDeletes((prev) => {
        const next = { ...prev };
        delete next[todo.id];
        return next;
      });
      delete timers.current[todo.id];
    }, UNDO_WINDOW_MS);
  }

  function handleUndo(id: string) {
    clearTimeout(timers.current[id]);
    delete timers.current[id];
    restoreTodo(id);
    setPendingDeletes((prev) => {
      const next = { ...prev };
      delete next[id];
      return next;
    });
  }

  return (
    <div className="todos-page">
      <header className="todos-header">
        <h1>Todos</h1>
      </header>

      <form className="todos-form" onSubmit={handleSubmit}>
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Add a todo…"
        />
        <button type="submit">Add</button>
      </form>

      <div className="todos-pills">
        {(["all", "pending", "completed"] as Filter[]).map((f) => (
          <button
            key={f}
            className={`pill ${filter === f ? "pill-active" : ""}`}
            onClick={() => setFilter(f)}
          >
            {f[0].toUpperCase() + f.slice(1)}{" "}
            <span className="pill-count">{counts[f]}</span>
          </button>
        ))}
      </div>

      <ul className="todos-list">
        {visible.map((todo) => (
          <li key={todo.id} className="todos-item">
            <input
              type="checkbox"
              checked={todo.completed}
              onChange={() => toggleTodo(todo.id)}
            />
            <span className={todo.completed ? "todo-done" : ""}>
              {todo.title}
            </span>
            <button className="todos-delete" onClick={() => handleDelete(todo)}>
              Delete
            </button>
          </li>
        ))}
        {visible.length === 0 && <p className="todos-empty">Nothing here.</p>}
      </ul>

      <div className="todos-undo-stack">
        {Object.values(pendingDeletes).map((todo) => (
          <div key={todo.id} className="undo-toast">
            <span>Deleted "{todo.title}"</span>
            <button onClick={() => handleUndo(todo.id)}>Undo</button>
          </div>
        ))}
      </div>
      <button
  className="fab"
  onClick={() => setModalTodoId("new")}
  aria-label="Add todo"
>
  +
</button>

{(modalTodoId === "new" || modalTodo) && (
  <TodoModal
    todo={modalTodoId === "new" ? null : modalTodo}
    onClose={() => setModalTodoId(null)}
    onSave={handleModalSave}
  />
)}
    </div>
  );
}