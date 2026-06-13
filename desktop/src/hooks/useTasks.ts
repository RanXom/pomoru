import { useState, useEffect, useCallback } from "react";
import { commands, type Task } from "../lib/commands";

export function useTasks() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [activeTaskTitle, setActiveTaskTitle] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    const snap = await commands.getState();
    setTasks(snap.tasks);
    setActiveTaskTitle(snap.active_task_title);
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const addTask = useCallback(async (title: string) => {
    await commands.addTask(title);
    await refresh();
  }, [refresh]);

  const editTask = useCallback(async (index: number, title: string) => {
    await commands.editTask(index, title);
    await refresh();
  }, [refresh]);

  const deleteTask = useCallback(async (index: number) => {
    await commands.deleteTask(index);
    await refresh();
  }, [refresh]);

  const toggleTask = useCallback(async (index: number) => {
    await commands.toggleTask(index);
    await refresh();
  }, [refresh]);

  const reorderTask = useCallback(async (index: number, direction: "up" | "down") => {
    await commands.reorderTask(index, direction);
    await refresh();
  }, [refresh]);

  const moveTask = useCallback(async (from: number, to: number) => {
    await commands.moveTask(from, to);
    await refresh();
  }, [refresh]);

  return {
    tasks,
    activeTaskTitle,
    addTask,
    editTask,
    deleteTask,
    toggleTask,
    reorderTask,
    moveTask,
    refresh,
  };
}
