import type { TimerState } from "../lib/commands";

interface ControlsProps {
  timer: TimerState;
  onToggle: () => void;
  onReset: () => void;
  onSkip: () => void;
}

export default function Controls({ timer, onToggle, onReset, onSkip }: ControlsProps) {
  return (
    <div className="controls" id="controls">
      <button
        className="control-btn secondary"
        onClick={onReset}
        aria-label="Reset timer"
        title="Reset (R)"
        id="btn-reset"
      >
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
          <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
          <path d="M3 3v5h5" />
        </svg>
      </button>

      <button
        className={`control-btn primary ${timer.mode} ${timer.is_running ? "running" : ""}`}
        onClick={onToggle}
        aria-label={timer.is_running ? "Pause" : "Start"}
        title="Space"
        id="btn-toggle"
      >
        {timer.is_running ? (
          <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
            <rect x="6" y="4" width="4" height="16" rx="1" />
            <rect x="14" y="4" width="4" height="16" rx="1" />
          </svg>
        ) : (
          <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
            <polygon points="6,3 20,12 6,21" />
          </svg>
        )}
      </button>

      <button
        className="control-btn secondary"
        onClick={onSkip}
        aria-label="Skip session"
        title="Skip (S)"
        id="btn-skip"
      >
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
          <polygon points="5,4 15,12 5,20" />
          <line x1="19" y1="5" x2="19" y2="19" />
        </svg>
      </button>
    </div>
  );
}
