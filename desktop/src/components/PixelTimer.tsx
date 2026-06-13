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

function formatMonolithicAscii(timeStr: string): string {
  const lines = ["", "", "", "", ""];
  for (let i = 0; i < timeStr.length; i++) {
    const char = timeStr[i];
    const art = ASCII_MAP[char] || ASCII_MAP[":"];
    for (let j = 0; j < 5; j++) {
      lines[j] += art[j];
      if (i < timeStr.length - 1) {
        lines[j] += "  ";
      }
    }
  }
  return lines.join("\n");
}

function formatTimeStr(secs: number): string {
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
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
  const asciiArt = formatMonolithicAscii(timeStr);

  const [currentAscii, setCurrentAscii] = useState(asciiArt);
  const [prevAscii, setPrevAscii] = useState("");
  const [isFading, setIsFading] = useState(false);

  useEffect(() => {
    if (asciiArt !== currentAscii) {
      setPrevAscii(currentAscii);
      setCurrentAscii(asciiArt);
      setIsFading(true);
      const timeout = setTimeout(() => setIsFading(false), 300);
      return () => clearTimeout(timeout);
    }
  }, [asciiArt, currentAscii]);

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
      <div style={{ position: "relative" }}>
        {isFading && (
          <pre key={`${prevAscii}-prev`} className={`ascii-art-timer ${timer.mode} fade-out`} style={{ position: "absolute", top: 0, left: 0, right: 0 }}>
            {prevAscii}
          </pre>
        )}
        <pre key={currentAscii} className={`ascii-art-timer ${timer.mode} ${isFading ? "fade-in" : ""}`}>
          {currentAscii}
        </pre>
      </div>
    </div>
  );
}
