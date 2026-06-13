import { useState, useEffect, useRef, useCallback } from "react";
import { commands, type TimerState, type TickResult } from "../lib/commands";
import { playWorkDoneSound, playBreakDoneSound } from "../lib/sounds";

export function useTimer() {
  const [timer, setTimer] = useState<TimerState | null>(null);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  // Load initial state
  useEffect(() => {
    commands.getState().then((snap) => setTimer(snap.timer));
  }, []);

  // 1-second tick interval when running
  useEffect(() => {
    if (timer?.is_running) {
      intervalRef.current = setInterval(async () => {
        const result: TickResult = await commands.tick();
        
        if (result.event) {
          if (result.timer.mode === "work") {
            playBreakDoneSound(); // Break finished, wake up for work
          } else {
            playWorkDoneSound(); // Work finished, relax for break
          }
        }
        
        setTimer(result.timer);
      }, 1000);
    } else {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    };
  }, [timer?.is_running]);

  const refreshTimer = useCallback(async () => {
    const snap = await commands.getState();
    setTimer(snap.timer);
  }, []);

  const toggleRunning = useCallback(async () => {
    await commands.toggleTimer();
    await refreshTimer();
  }, [refreshTimer]);

  const reset = useCallback(async () => {
    await commands.resetTimer();
    await refreshTimer();
  }, [refreshTimer]);

  const skip = useCallback(async () => {
    await commands.skipSession();
    await refreshTimer();
  }, [refreshTimer]);

  const cycleMode = useCallback(async () => {
    await commands.cycleMode();
    await refreshTimer();
  }, [refreshTimer]);

  const toggleAutoSwitch = useCallback(async () => {
    await commands.toggleAutoSwitch();
    await refreshTimer();
  }, [refreshTimer]);

  const setDuration = useCallback(async (mins: number) => {
    await commands.setDuration(mins);
    await refreshTimer();
  }, [refreshTimer]);

  return {
    timer,
    toggleRunning,
    reset,
    skip,
    cycleMode,
    toggleAutoSwitch,
    setDuration,
  };
}
