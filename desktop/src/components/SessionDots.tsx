import type { TimerState } from "../lib/commands";

interface SessionDotsProps {
  timer: TimerState;
  onCycleMode: () => void;
}

const modes: Array<{ key: string; label: string }> = [
  { key: "work", label: "Focus" },
  { key: "short-break", label: "Short Break" },
  { key: "long-break", label: "Long Break" },
];

export default function SessionDots({ timer, onCycleMode }: SessionDotsProps) {
  return (
    <div className="session-dots" id="session-dots">
      {modes.map((m) => (
        <button
          key={m.key}
          className={`session-dot ${timer.mode === m.key ? "active" : ""}`}
          onClick={onCycleMode}
          disabled={timer.is_running}
          aria-label={`Switch to ${m.label}`}
          title={timer.is_running ? "Pause to switch" : `Switch to ${m.label}`}
        >
          {timer.mode === m.key && <span className="dot-indicator" />}
          {m.label}
        </button>
      ))}
    </div>
  );
}
