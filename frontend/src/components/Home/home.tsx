"use client";

import { useTodos } from "@/lib/hooks/use-todos";
import "./home.css";
import { ArrowUpRight } from "lucide-react";
import Link from "next/link";

export function Home() {
  const { data: todos, isLoading } = useTodos();

  const pending = todos?.filter((t) => !t.completed) ?? [];

  return (
    <div className="home">
      <header className="home-header">
        <h1>Home</h1>
        <p className="home-subtitle">Greetings FlasH</p>
      </header>

      <section className="home-section">
        <div className="home-section-header">
          <h2>Todos</h2>
          <span className="home-count">
            {isLoading ? "…" : pending.length} open
          </span>
          <Link href="/todos">
            <button className="card-link">
              <ArrowUpRight size={25} />
            </button>
          </Link>
        </div>

        {isLoading ? (
          <p className="home-empty">Loading…</p>
        ) : pending.length === 0 ? (
          <p className="home-empty">
            Nothing pending. Add one on the Todos page.
          </p>
        ) : (
          <ul className="home-list">
            {pending.slice(0, 5).map((todo) => (
              <li key={todo.id}>{todo.title}</li>
            ))}
          </ul>
        )}
      </section>

      <section className="home-section">
        <div className="home-section-header">
          <h2>Notes</h2>
          <Link href="/notes">
            <button className="card-link">
              <ArrowUpRight size={25} />
            </button>
          </Link>
        </div>
        <p className="home-empty">Not set up yet.</p>
      </section>

      <section className="home-section">
        <div className="home-section-header">
          <h2>Projects</h2>
          <Link href="/projects">
            <button className="card-link">
              <ArrowUpRight size={25} />
            </button>
          </Link>
        </div>
        <p className="home-empty">Not set up yet.</p>
      </section>
    </div>
  );
}
