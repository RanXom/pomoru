import { useEffect, useState } from "react";
import { useTimer } from "./hooks/useTimer";
import { useTasks } from "./hooks/useTasks";
import { commands } from "./lib/commands";
import Titlebar from "./components/Titlebar";
import Header from "./components/Header";
import PixelTimer from "./components/PixelTimer";
import SessionDots from "./components/SessionDots";
import Controls from "./components/Controls";
import TaskList from "./components/TaskList";
import "./App.css";

export default function App() {
  const [isSidebarOpen, setIsSidebarOpen] = useState(false);

  const {
    timer,
    toggleRunning,
    reset,
    skip,
    cycleMode,
    toggleAutoSwitch,
    setDuration,
  } = useTimer();

  const {
    tasks,
    activeTaskTitle,
    addTask,
    editTask,
    deleteTask,
    toggleTask,
    reorderTask,
    moveTask,
  } = useTasks();

  // Load theme on mount
  useEffect(() => {
    commands.getTheme().then((theme) => {
      const root = document.documentElement;
      root.style.setProperty("--primary", theme.primary);
      root.style.setProperty("--overlay", theme.overlay0);
      root.style.setProperty("--surface", theme.surface0);
      root.style.setProperty("--text", theme.text);
    });
  }, []);

  // Global keyboard shortcuts
  useEffect(() => {
    function handleKeyDown(e: KeyboardEvent) {
      // Don't capture when typing in inputs
      const target = e.target as HTMLElement;
      if (target.tagName === "INPUT" || target.tagName === "TEXTAREA") return;

      switch (e.code) {
        case "Space":
          e.preventDefault();
          toggleRunning();
          break;
        case "KeyR":
          reset();
          break;
        case "KeyS":
          skip();
          break;
        case "Tab":
          e.preventDefault();
          cycleMode();
          break;
        case "KeyA":
          toggleAutoSwitch();
          break;
      }
    }

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [toggleRunning, reset, skip, cycleMode, toggleAutoSwitch]);

  if (!timer) {
    return (
      <div className="app loading">
        <div className="loading-text">Loading...</div>
      </div>
    );
  }

  return (
    <div className="app" id="app">
      <Titlebar
        isSidebarOpen={isSidebarOpen}
        onToggleSidebar={() => setIsSidebarOpen(!isSidebarOpen)}
      />
      
      <main className="app-layout">
        {/* Sliding Sidebar for Tasks */}
        <section className={`sidebar ${isSidebarOpen ? "open" : ""}`} aria-label="Tasks Sidebar">
          <TaskList
            tasks={tasks}
            onAdd={addTask}
            onEdit={editTask}
            onDelete={deleteTask}
            onToggle={toggleTask}
            onReorder={reorderTask}
            onMove={moveTask}
          />
        </section>

        {/* Main Timer View */}
        <section className="main-view" aria-label="Timer">
          <div className="timer-content">
            <Header timer={timer} activeTaskTitle={activeTaskTitle} />
            <PixelTimer timer={timer} onSetDuration={setDuration} />
            <SessionDots timer={timer} onCycleMode={cycleMode} />
            <Controls
              timer={timer}
              onToggle={toggleRunning}
              onReset={reset}
              onSkip={skip}
            />
          </div>
          
          <div className="timer-footer">
            <button
              className={`auto-switch-toggle ${timer.auto_switch_sessions ? "active" : ""}`}
              onClick={toggleAutoSwitch}
              aria-label="Toggle auto-switch"
              title="Toggle auto-switch (A)"
            >
              {timer.auto_switch_sessions ? "Auto" : "Manual"}
            </button>
            <span className="shortcut-hints">
              Space · R · S · Tab · A
            </span>
          </div>
        </section>
      </main>
    </div>
  );
}
