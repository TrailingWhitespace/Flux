// src/components/todo-modal.tsx
"use client";

import { useEffect, useState } from "react";
import type { Todo, TodoInput } from "@/lib/api/todos";
import "./todo_modal.css";

const PRIORITIES = [
  { value: 0, label: "None" },
  { value: 1, label: "Low" },
  { value: 2, label: "Medium" },
  { value: 3, label: "High" },
];

function toDateInputValue(ts: number | null): string {
  if (!ts) return "";
  return new Date(ts).toISOString().slice(0, 10); // "YYYY-MM-DD"
}

function fromDateInputValue(value: string): number | undefined {
  if (!value) return undefined;
  return new Date(value).getTime();
}

export function TodoModal({
  todo, // null = create mode, Todo = edit mode
  onClose,
  onSave,
}: {
  todo: Todo | null;
  onClose: () => void;
  onSave: (input: TodoInput) => void;
}) {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [priority, setPriority] = useState(0);
  const [dueDate, setDueDate] = useState("");

  // whenever the modal opens (or the target todo changes), reset the draft to match it
  useEffect(() => {
    setTitle(todo?.title ?? "");
    setDescription(todo?.description ?? "");
    setPriority(todo?.priority ?? 0);
    setDueDate(toDateInputValue(todo?.dueDate ?? null));
  }, [todo]);

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!title.trim()) return;
    onSave({
      title: title.trim(),
      description: description.trim() || undefined,
      priority,
      dueDate: fromDateInputValue(dueDate),
    });
    onClose();
  }

  return (
    <div
      className="modal-overlay"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <form className="modal-content" onSubmit={handleSubmit}>
        <h2>{todo ? "Edit Todo" : "New Todo"}</h2>

        <input
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Title"
          autoFocus
        />

        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="Description"
          rows={4}
        />

        <select
          value={priority}
          onChange={(e) => setPriority(Number(e.target.value))}
        >
          {PRIORITIES.map((p) => (
            <option key={p.value} value={p.value}>
              {p.label}
            </option>
          ))}
        </select>

        <input
          type="date"
          value={dueDate}
          onChange={(e) => setDueDate(e.target.value)}
        />

        <div className="modal-actions">
          <button type="button" onClick={onClose}>
            Cancel
          </button>
          <button type="submit">Save</button>
        </div>
      </form>
    </div>
  );
}