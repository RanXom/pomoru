import { useState, useRef, useEffect } from "react";
import type { TimerState } from "../lib/commands";

interface PixelTimerProps {
  timer: TimerState;
  onSetDuration: (mins: number) => void;
}

const ASCII_MAP: Record<string, string[]> = {
  "0": [" ██████ ", "██    ██", "██    ██", "██    ██", " ██████ "],
  "1": ["   ██   ", "  ███   ", "   ██   ", "   ██   ", " ██████ "],
  "2": [" ██████ ", "██    ██", "    ███ ", "  ███   ", "████████"],
  "3": [" ██████ ", "      ██", "  █████ ", "      ██", " ██████ "],
  "4": ["██    ██", "██    ██", "████████", "      ██", "      ██"],
  "5": ["████████", "██      ", "███████ ", "      ██", "███████ "],
  "6": [" ██████ ", "██      ", "███████ ", "██    ██", " ██████ "],
  "7": ["████████", "      ██", "     ██ ", "    ██  ", "   ██   "],
  "8": [" ██████ ", "██    ██", " ██████ ", "██    ██", " ██████ "],
  "9": [" ██████ ", "██    ██", " ███████", "      ██", " ██████ "],
  ":": ["        ", "   █    ", "        ", "   █    ", "        "],
};

function formatCharAscii(char: string): string {
  if (!char) return "";
  const art = ASCII_MAP[char] || ASCII_MAP[":"];
  return art.join("\n");
}

function formatTimeStr(secs: number): string {
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}

function AnimatedDigit({ char, mode }: { char: string; mode: string }) {
  const [current, setCurrent] = useState(char);
  const [prev, setPrev] = useState("");
  const [isAnimating, setIsAnimating] = useState(false);

  useEffect(() => {
    if (char !== current) {
      setPrev(current);
      setCurrent(char);
      setIsAnimating(true);
      const timeout = setTimeout(() => setIsAnimating(false), 300);
      return () => clearTimeout(timeout);
    }
  }, [char, current]);

  const ascii = formatCharAscii(current);
  const prevAscii = formatCharAscii(prev);

  return (
    <div style={{ position: "relative", overflow: "hidden", display: "inline-block" }}>
      {isAnimating && prev && (
        <pre key={`${prev}-prev`} className={`ascii-art-timer ${mode} scroll-out`} style={{ position: "absolute", top: 0, left: 0, right: 0 }}>
          {prevAscii}
        </pre>
      )}
      <pre key={current} className={`ascii-art-timer ${mode} ${isAnimating ? "scroll-in" : ""}`}>
        {ascii}
      </pre>
    </div>
  );
}

export default function PixelTimer({ timer, onSetDuration }: PixelTimerProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [editValue, setEditValue] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isEditing && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isEditing]);

  const timeStr = formatTimeStr(timer.time_remaining_secs);

  function handleStartEdit() {
    setEditValue(String(Math.floor(timer.time_remaining_secs / 60)));
    setIsEditing(true);
  }

  function handleSubmit() {
    const mins = parseInt(editValue, 10);
    if (!isNaN(mins) && mins >= 0) {
      onSetDuration(mins);
    }
    setIsEditing(false);
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === "Enter") {
      handleSubmit();
    } else if (e.key === "Escape") {
      setIsEditing(false);
    }
  }

  if (isEditing) {
    return (
      <div className="pixel-timer-container editing">
        <input
          ref={inputRef}
          type="number"
          className="timer-edit-input"
          value={editValue}
          onChange={(e) => setEditValue(e.target.value)}
          onBlur={handleSubmit}
          onKeyDown={handleKeyDown}
          min="0"
          max="999"
          aria-label="Set minutes"
        />
        <span className="timer-edit-label">minutes</span>
      </div>
    );
  }

  return (
    <div
      className="pixel-timer-container"
      onClick={handleStartEdit}
      title="Click to edit timer"
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") handleStartEdit();
      }}
    >
      <div style={{ display: "flex", gap: "12px", alignItems: "center" }}>
        {timeStr.split("").map((char, index) => (
          <AnimatedDigit key={index} char={char} mode={timer.mode} />
        ))}
      </div>
    </div>
  );
}
