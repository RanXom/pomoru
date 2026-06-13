import { useState } from "react";
import type { Task } from "../lib/commands";
import TaskItem from "./TaskItem";

interface TaskListProps {
  tasks: Task[];
  onAdd: (title: string) => void;
  onEdit: (index: number, title: string) => void;
  onDelete: (index: number) => void;
  onToggle: (index: number) => void;
  onReorder: (index: number, direction: "up" | "down") => void;
  onMove: (from: number, to: number) => void;
}

export default function TaskList({
  tasks,
  onAdd,
  onEdit,
  onDelete,
  onToggle,
  onReorder,
  onMove,
}: TaskListProps) {
  const [newTaskTitle, setNewTaskTitle] = useState("");
  const [draggedIndex, setDraggedIndex] = useState<number | null>(null);
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (newTaskTitle.trim()) {
      onAdd(newTaskTitle.trim());
      setNewTaskTitle("");
    }
  }

  function handleDragStart(e: React.DragEvent, index: number) {
    setDraggedIndex(index);
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = "move";
      e.dataTransfer.setData("text/plain", index.toString());

      // Generate a small, clean drag ghost without relying on the browser's 
      // unpredictable DOM snapshotting on High DPI displays.
      const task = tasks[index];
      const ghost = document.createElement("div");
      ghost.textContent = task.title;
      ghost.style.position = "absolute";
      ghost.style.top = "-1000px";
      ghost.style.left = "-1000px";
      ghost.style.backgroundColor = "var(--surface)";
      ghost.style.color = "var(--text)";
      ghost.style.padding = "4px 8px";
      ghost.style.borderRadius = "var(--radius-sm)";
      // WebKit scaling fix: Make it 12px natively so it stays small even if scaled 2x
      ghost.style.fontSize = "12px";
      ghost.style.fontFamily = "inherit";
      ghost.style.whiteSpace = "nowrap";
      ghost.style.boxShadow = "0 4px 12px rgba(0,0,0,0.5)";
      ghost.style.zIndex = "9999";

      document.body.appendChild(ghost);
      e.dataTransfer.setDragImage(ghost, 10, 10);

      setTimeout(() => {
        if (document.body.contains(ghost)) {
          document.body.removeChild(ghost);
        }
      }, 0);
    }
  }

  function handleDragOver(e: React.DragEvent, index: number) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = "move";
    }
    if (draggedIndex !== null && draggedIndex !== index) {
      setDragOverIndex(index);
    }
  }

  function handleDrop(e: React.DragEvent, dropIndex: number) {
    e.preventDefault();
    if (draggedIndex !== null && draggedIndex !== dropIndex) {
      onMove(draggedIndex, dropIndex);
    }
    setDraggedIndex(null);
    setDragOverIndex(null);
  }

  function handleDragEnd() {
    setDraggedIndex(null);
    setDragOverIndex(null);
  }

  return (
    <div className="task-list-container">
      <div className="task-list-header">
        <h2 className="task-list-title">Focus Priorities</h2>
        <span className="task-count">
          {tasks.filter((t) => !t.is_done).length}/{tasks.length}
        </span>
      </div>

      <form className="task-input-row" onSubmit={handleSubmit}>
        <input
          type="text"
          className="task-input"
          placeholder="Add a new task..."
          value={newTaskTitle}
          onChange={(e) => setNewTaskTitle(e.target.value)}
        />
        <button
          type="submit"
          className="task-add-btn"
          disabled={!newTaskTitle.trim()}
          aria-label="Add task"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round">
            <line x1="12" y1="5" x2="12" y2="19" />
            <line x1="5" y1="12" x2="19" y2="12" />
          </svg>
        </button>
      </form>

      <div className="task-items">
        {tasks.map((task, index) => {
          let transform = "none";
          if (draggedIndex !== null && dragOverIndex !== null) {
            if (draggedIndex < dragOverIndex && index > draggedIndex && index <= dragOverIndex) {
              transform = "translateY(-44px)";
            } else if (draggedIndex > dragOverIndex && index >= dragOverIndex && index < draggedIndex) {
              transform = "translateY(44px)";
            }
          }

          return (
            <div
              key={`${task.title}-${index}`}
              draggable
              onDragStart={(e) => handleDragStart(e, index)}
              onDragOver={(e) => handleDragOver(e, index)}
              onDrop={(e) => handleDrop(e, index)}
              onDragEnd={handleDragEnd}
              className={`task-drag-wrapper ${draggedIndex === index ? "dragging" : ""}`}
              style={{
                transform,
                transition: draggedIndex !== null ? "transform 0.2s cubic-bezier(0.2, 0, 0, 1)" : "none",
              }}
            >
              <TaskItem
                task={task}
                index={index}
                total={tasks.length}
                onEdit={onEdit}
                onDelete={onDelete}
                onToggle={onToggle}
                onReorder={onReorder}
              />
            </div>
          );
        })}
        {tasks.length === 0 && (
          <div className="task-empty">
            <p>No tasks yet</p>
            <span className="task-empty-hint">Time to focus!</span>
          </div>
        )}
      </div>
    </div>
  );
}
