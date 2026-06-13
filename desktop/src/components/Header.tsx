import type { TimerState } from "../lib/commands";

interface HeaderProps {
  timer: TimerState;
  activeTaskTitle: string | null;
}

function modeLabel(mode: string): string {
  switch (mode) {
    case "work": return "Focus Session";
    case "short-break": return "Short Break";
    case "long-break": return "Long Break";
    default: return "Session";
  }
}

function formatTime(secs: number): string {
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}

export default function Header({ timer, activeTaskTitle }: HeaderProps) {
  return (
    <div className="header" id="header">
      <div className="header-session">
        <span className={`session-badge ${timer.mode}`}>
          {modeLabel(timer.mode)}
        </span>
      </div>
      <div className="header-time">{formatTime(timer.time_remaining_secs)}</div>
      {activeTaskTitle && (
        <div className="header-task">
          <span className="task-indicator" />
          {activeTaskTitle}
        </div>
      )}
    </div>
  );
}
