import { invoke } from "@tauri-apps/api/core";

// --- Types ---

export interface TimerState {
  mode: "work" | "short-break" | "long-break";
  work_time_secs: number;
  short_break_secs: number;
  long_break_secs: number;
  time_remaining_secs: number;
  total_duration_secs: number;
  is_running: boolean;
  auto_switch_sessions: boolean;
  break_count: number;
}

export interface Task {
  title: string;
  is_done: boolean;
}

export interface TimerEvent {
  title: string;
  message: string;
}

export interface AppSnapshot {
  timer: TimerState;
  tasks: Task[];
  selected_task: number | null;
  active_task_title: string | null;
}

export interface TickResult {
  timer: TimerState;
  event: TimerEvent | null;
}

export interface ThemeColors {
  primary: string;
  overlay0: string;
  surface0: string;
  text: string;
}

// --- Commands ---

export const commands = {
  getState: () => invoke<AppSnapshot>("get_state"),
  tick: () => invoke<TickResult>("tick"),
  startTimer: () => invoke<void>("start_timer"),
  pauseTimer: () => invoke<void>("pause_timer"),
  toggleTimer: () => invoke<void>("toggle_timer"),
  resetTimer: () => invoke<void>("reset_timer"),
  skipSession: () => invoke<void>("skip_session"),
  cycleMode: () => invoke<void>("cycle_mode"),
  toggleAutoSwitch: () => invoke<void>("toggle_auto_switch"),
  setDuration: (mins: number) => invoke<void>("set_duration", { mins }),
  getTasks: () => invoke<Task[]>("get_tasks"),
  addTask: (title: string) => invoke<void>("add_task", { title }),
  editTask: (index: number, title: string) => invoke<void>("edit_task", { index, title }),
  deleteTask: (index: number) => invoke<void>("delete_task", { index }),
  toggleTask: (index: number) => invoke<void>("toggle_task", { index }),
  reorderTask: (index: number, direction: "up" | "down") =>
    invoke<void>("reorder_task", { index, direction }),
  moveTask: (from: number, to: number) => invoke<void>("move_task", { from, to }),
  saveConfig: () => invoke<void>("save_config"),
  getTheme: () => invoke<ThemeColors>("get_theme"),
};
