import { useState, useRef, useEffect } from "react";
import type { Task } from "../lib/commands";

interface TaskItemProps {
  task: Task;
  index: number;
  total: number;
  onEdit: (index: number, title: string) => void;
  onDelete: (index: number) => void;
  onToggle: (index: number) => void;
  onReorder: (index: number, direction: "up" | "down") => void;
}

export default function TaskItem({
  task,
  index,
  total: _total,
  onEdit,
  onDelete,
  onToggle,
  onReorder: _onReorder,
}: TaskItemProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [editTitle, setEditTitle] = useState(task.title);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isEditing && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isEditing]);

  function handleSubmit() {
    if (editTitle.trim() && editTitle !== task.title) {
      onEdit(index, editTitle.trim());
    } else {
      setEditTitle(task.title);
    }
    setIsEditing(false);
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === "Enter") {
      handleSubmit();
    } else if (e.key === "Escape") {
      setEditTitle(task.title);
      setIsEditing(false);
    }
  }

  return (
    <div className={`task-item ${task.is_done ? "done" : ""}`}>
      <button
        className="task-checkbox"
        onClick={() => onToggle(index)}
        aria-label={task.is_done ? "Mark incomplete" : "Mark complete"}
      >
        {task.is_done ? (
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
            <path d="M9 12l2 2 4-4" />
          </svg>
        ) : (
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
          </svg>
        )}
      </button>

      {isEditing ? (
        <input
          ref={inputRef}
          type="text"
          className="task-edit-input"
          value={editTitle}
          onChange={(e) => setEditTitle(e.target.value)}
          onBlur={handleSubmit}
          onKeyDown={handleKeyDown}
        />
      ) : (
        <div className="task-title" onDoubleClick={() => setIsEditing(true)}>
          <span className="task-title-text">{task.title}</span>
        </div>
      )}

      <div className="task-actions">
        <button
          className="task-action-btn edit"
          onClick={() => setIsEditing(true)}
          aria-label="Edit task"
          title="Edit"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M12 20h9" />
            <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" />
          </svg>
        </button>
        <button
          className="task-action-btn delete"
          onClick={() => onDelete(index)}
          aria-label="Delete task"
          title="Delete"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <polyline points="3 6 5 6 21 6" />
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
          </svg>
        </button>
      </div>
    </div>
  );
}
