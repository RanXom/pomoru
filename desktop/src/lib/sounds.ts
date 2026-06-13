let audioCtx: AudioContext | null = null;

function getAudioContext() {
  if (!audioCtx) {
    audioCtx = new (window.AudioContext || (window as any).webkitAudioContext)();
  }
  return audioCtx;
}

export function playWorkDoneSound() {
  const ctx = getAudioContext();
  if (ctx.state === "suspended") ctx.resume();

  const now = ctx.currentTime;

  // Relaxing, dreamy Cmaj9 chord (C4, E4, G4, B4, D5)
  // Strummed slowly like a soft kalimba or meditation bowl
  const notes = [261.63, 329.63, 392.00, 493.88, 587.33];
  notes.forEach((freq, i) => {
    playTone(ctx, freq, now + i * 0.06, 4.0);
  });
}

export function playBreakDoneSound() {
  const ctx = getAudioContext();
  if (ctx.state === "suspended") ctx.resume();

  const now = ctx.currentTime;

  // Bright, uplifting Fmaj9 chord (F4, A4, C5, E5, G5)
  const notes = [349.23, 440.00, 523.25, 659.25, 783.99];
  notes.forEach((freq, i) => {
    playTone(ctx, freq, now + i * 0.06, 4.0);
  });
}

function playTone(ctx: AudioContext, frequency: number, startTime: number, duration: number) {
  // 1. Fundamental body (warm sine)
  const osc1 = ctx.createOscillator();
  const gain1 = ctx.createGain();
  osc1.type = "sine";
  osc1.frequency.setValueAtTime(frequency, startTime);
  
  gain1.gain.setValueAtTime(0, startTime);
  // Soft attack, long decay (Volume drastically reduced to support chords without clipping)
  gain1.gain.linearRampToValueAtTime(0.08, startTime + 0.05);
  gain1.gain.exponentialRampToValueAtTime(0.001, startTime + duration);
  
  osc1.connect(gain1);
  gain1.connect(ctx.destination);
  
  osc1.start(startTime);
  osc1.stop(startTime + duration);

  // 2. Kalimba metallic overtone (inharmonic triangle)
  const osc2 = ctx.createOscillator();
  const gain2 = ctx.createGain();
  osc2.type = "triangle";
  osc2.frequency.setValueAtTime(frequency * 3.14, startTime);
  
  gain2.gain.setValueAtTime(0, startTime);
  // Fast attack, short decay for the woody "tink"
  gain2.gain.linearRampToValueAtTime(0.02, startTime + 0.01);
  gain2.gain.exponentialRampToValueAtTime(0.001, startTime + 0.3);
  
  osc2.connect(gain2);
  gain2.connect(ctx.destination);
  
  osc2.start(startTime);
  osc2.stop(startTime + 0.3);

  // 3. Thumb pluck transient (high frequency click)
  const osc3 = ctx.createOscillator();
  const gain3 = ctx.createGain();
  osc3.type = "sine";
  osc3.frequency.setValueAtTime(frequency * 6.5, startTime);
  
  gain3.gain.setValueAtTime(0, startTime);
  gain3.gain.linearRampToValueAtTime(0.01, startTime + 0.002);
  gain3.gain.exponentialRampToValueAtTime(0.001, startTime + 0.05);
  
  osc3.connect(gain3);
  gain3.connect(ctx.destination);
  
  osc3.start(startTime);
  osc3.stop(startTime + 0.05);
}
