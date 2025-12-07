import { getContext, setContext } from "svelte";
import { scene } from "$lib/stores";
import { snapGranularity } from "$lib/stores/snapGranularity";
import { setFrames, ActionTiming } from "$lib/api/client";
import type { Frame } from "$lib/types/protocol";
import { get } from "svelte/store";

const TIMELINE_CONTEXT_KEY = "timeline";

// Types
export interface EditingState {
  lineIdx: number;
  frameIdx: number;
  value: string;
}

export interface ResizeState {
  lineIdx: number;
  frameIdx: number;
  startPos: number;
  startDuration: number;
  previewDuration: number;
}

export interface DragState {
  sourceLineIdx: number;
  sourceFrameIdx: number;
  frame: Frame;
  currentLineIdx: number;
  currentFrameIdx: number;
}

export interface TimelineContext {
  // Layout
  pixelsPerBeat: number;
  trackSize: number;
  isVertical: boolean;

  // Editing state
  editingDuration: EditingState | null;
  editingReps: EditingState | null;
  editingName: EditingState | null;

  // Resize state
  resizing: ResizeState | null;

  // Drag state
  dragging: DragState | null;

  // Editing actions
  startDurationEdit: (lineIdx: number, frameIdx: number) => void;
  startRepsEdit: (lineIdx: number, frameIdx: number) => void;
  startNameEdit: (lineIdx: number, frameIdx: number) => void;
  updateEditValue: (type: "duration" | "reps" | "name", value: string) => void;
  commitEdit: (type: "duration" | "reps" | "name", shiftKey?: boolean) => Promise<void>;
  cancelEdit: () => void;
  isEditing: () => boolean;

  // Resize actions
  startResize: (lineIdx: number, frameIdx: number, event: PointerEvent) => void;
  getPreviewDuration: (lineIdx: number, frameIdx: number) => number | null;

  // Drag actions
  startDrag: (lineIdx: number, frameIdx: number) => void;
  getDropIndicatorIdx: (lineIdx: number) => number | null;
}

export function getDuration(frame: Frame): number {
  const d = frame.duration;
  return typeof d === "number" && !isNaN(d) && d > 0 ? d : 1;
}

function getReps(frame: Frame): number {
  const r = frame.repetitions;
  return typeof r === "number" && !isNaN(r) && r >= 1 ? r : 1;
}

export function createTimelineContext(initial: {
  pixelsPerBeat: number;
  trackSize: number;
  isVertical: boolean;
}): TimelineContext {
  // Layout state
  let pixelsPerBeat = $state(initial.pixelsPerBeat);
  let trackSize = $state(initial.trackSize);
  let isVertical = $state(initial.isVertical);

  // Editing state
  let editingDuration = $state<EditingState | null>(null);
  let editingReps = $state<EditingState | null>(null);
  let editingName = $state<EditingState | null>(null);

  // Resize state
  let resizing = $state<ResizeState | null>(null);

  // Drag state
  let dragging = $state<DragState | null>(null);

  // Editing actions
  function startDurationEdit(lineIdx: number, frameIdx: number) {
    const currentScene = get(scene);
    if (!currentScene) return;
    const frame = currentScene.lines[lineIdx]?.frames[frameIdx];
    if (!frame) return;
    editingDuration = {
      lineIdx,
      frameIdx,
      value: getDuration(frame).toString(),
    };
  }

  function startRepsEdit(lineIdx: number, frameIdx: number) {
    const currentScene = get(scene);
    if (!currentScene) return;
    const frame = currentScene.lines[lineIdx]?.frames[frameIdx];
    if (!frame) return;
    editingReps = {
      lineIdx,
      frameIdx,
      value: getReps(frame).toString(),
    };
  }

  function startNameEdit(lineIdx: number, frameIdx: number) {
    const currentScene = get(scene);
    if (!currentScene) return;
    const frame = currentScene.lines[lineIdx]?.frames[frameIdx];
    if (!frame) return;
    editingName = {
      lineIdx,
      frameIdx,
      value: frame.name || "",
    };
  }

  function updateEditValue(type: "duration" | "reps" | "name", value: string) {
    if (type === "duration" && editingDuration) {
      editingDuration = { ...editingDuration, value };
    } else if (type === "reps" && editingReps) {
      editingReps = { ...editingReps, value };
    } else if (type === "name" && editingName) {
      editingName = { ...editingName, value };
    }
  }

  async function commitEdit(type: "duration" | "reps" | "name", shiftKey = false) {
    const currentScene = get(scene);
    if (!currentScene) return;

    const snap = get(snapGranularity);

    if (type === "duration" && editingDuration) {
      const parsed = parseFloat(editingDuration.value);
      if (!isNaN(parsed) && parsed > 0) {
        const snapValue = shiftKey ? snap / 2 : snap;
        const newDuration = Math.max(snapValue, Math.round(parsed / snapValue) * snapValue);
        const frame = currentScene.lines[editingDuration.lineIdx]?.frames[editingDuration.frameIdx];
        if (frame) {
          const updatedFrame = { ...frame, duration: newDuration };
          try {
            await setFrames(
              [[editingDuration.lineIdx, editingDuration.frameIdx, updatedFrame]],
              ActionTiming.immediate()
            );
          } catch (error) {
            console.error("Failed to update duration:", error);
          }
        }
      }
      editingDuration = null;
    } else if (type === "reps" && editingReps) {
      const parsed = parseInt(editingReps.value, 10);
      if (!isNaN(parsed) && parsed >= 1) {
        const frame = currentScene.lines[editingReps.lineIdx]?.frames[editingReps.frameIdx];
        if (frame) {
          const updatedFrame = { ...frame, repetitions: parsed };
          try {
            await setFrames(
              [[editingReps.lineIdx, editingReps.frameIdx, updatedFrame]],
              ActionTiming.immediate()
            );
          } catch (error) {
            console.error("Failed to update repetitions:", error);
          }
        }
      }
      editingReps = null;
    } else if (type === "name" && editingName) {
      const frame = currentScene.lines[editingName.lineIdx]?.frames[editingName.frameIdx];
      if (frame) {
        const newName = editingName.value.trim() || null;
        const updatedFrame = { ...frame, name: newName };
        try {
          await setFrames(
            [[editingName.lineIdx, editingName.frameIdx, updatedFrame]],
            ActionTiming.immediate()
          );
        } catch (error) {
          console.error("Failed to update name:", error);
        }
      }
      editingName = null;
    }
  }

  function cancelEdit() {
    editingDuration = null;
    editingReps = null;
    editingName = null;
  }

  function isEditing(): boolean {
    return editingDuration !== null || editingReps !== null || editingName !== null;
  }

  // Resize actions
  function startResize(lineIdx: number, frameIdx: number, event: PointerEvent) {
    event.stopPropagation();
    event.preventDefault();
    const currentScene = get(scene);
    if (!currentScene) return;
    const line = currentScene.lines[lineIdx];
    if (!line) return;
    const frame = line.frames[frameIdx];
    if (!frame) return;
    const duration = getDuration(frame);

    const startPos = isVertical ? event.clientY : event.clientX;

    resizing = {
      lineIdx,
      frameIdx,
      startPos,
      startDuration: duration,
      previewDuration: duration,
    };

    (event.target as HTMLElement).setPointerCapture(event.pointerId);
    window.addEventListener("pointermove", handleResizeMove);
    window.addEventListener("pointerup", handleResizeEnd);
  }

  function handleResizeMove(event: PointerEvent) {
    if (!resizing) return;
    const currentScene = get(scene);
    if (!currentScene) return;
    const line = currentScene.lines[resizing.lineIdx];
    if (!line) return;
    const frame = line.frames[resizing.frameIdx];
    if (!frame) return;

    const snap = event.shiftKey ? get(snapGranularity) / 2 : get(snapGranularity);
    const currentPos = isVertical ? event.clientY : event.clientX;
    const delta = currentPos - resizing.startPos;
    const reps = getReps(frame);
    const deltaDuration = delta / pixelsPerBeat / reps;
    const newDuration = Math.max(
      snap,
      Math.round((resizing.startDuration + deltaDuration) / snap) * snap
    );

    resizing = { ...resizing, previewDuration: newDuration };
  }

  async function handleResizeEnd() {
    window.removeEventListener("pointermove", handleResizeMove);
    window.removeEventListener("pointerup", handleResizeEnd);

    if (!resizing) return;
    const currentScene = get(scene);
    if (!currentScene) {
      resizing = null;
      return;
    }

    const line = currentScene.lines[resizing.lineIdx];
    if (!line) {
      resizing = null;
      return;
    }
    const frame = line.frames[resizing.frameIdx];
    if (!frame) {
      resizing = null;
      return;
    }

    const newDuration = resizing.previewDuration;
    if (newDuration !== getDuration(frame)) {
      const updatedFrame = { ...frame, duration: newDuration };
      try {
        await setFrames(
          [[resizing.lineIdx, resizing.frameIdx, updatedFrame]],
          ActionTiming.immediate()
        );
      } catch (error) {
        console.error("Failed to update frame duration:", error);
      }
    }
    resizing = null;
  }

  function getPreviewDuration(lineIdx: number, frameIdx: number): number | null {
    if (resizing && resizing.lineIdx === lineIdx && resizing.frameIdx === frameIdx) {
      return resizing.previewDuration;
    }
    return null;
  }

  // Drag actions
  function startDrag(lineIdx: number, frameIdx: number) {
    const currentScene = get(scene);
    if (!currentScene) return;
    const frame = currentScene.lines[lineIdx]?.frames[frameIdx];
    if (!frame) return;

    dragging = {
      sourceLineIdx: lineIdx,
      sourceFrameIdx: frameIdx,
      frame: structuredClone(frame),
      currentLineIdx: lineIdx,
      currentFrameIdx: frameIdx,
    };
  }

  function getDropIndicatorIdx(lineIdx: number): number | null {
    if (!dragging || dragging.currentLineIdx !== lineIdx) return null;
    return dragging.currentFrameIdx;
  }

  const ctx: TimelineContext = {
    get pixelsPerBeat() { return pixelsPerBeat; },
    set pixelsPerBeat(v) { pixelsPerBeat = v; },
    get trackSize() { return trackSize; },
    set trackSize(v) { trackSize = v; },
    get isVertical() { return isVertical; },
    set isVertical(v) { isVertical = v; },
    get editingDuration() { return editingDuration; },
    get editingReps() { return editingReps; },
    get editingName() { return editingName; },
    get resizing() { return resizing; },
    set resizing(v) { resizing = v; },
    get dragging() { return dragging; },
    set dragging(v) { dragging = v; },
    startDurationEdit,
    startRepsEdit,
    startNameEdit,
    updateEditValue,
    commitEdit,
    cancelEdit,
    isEditing,
    startResize,
    getPreviewDuration,
    startDrag,
    getDropIndicatorIdx,
  };

  setContext(TIMELINE_CONTEXT_KEY, ctx);
  return ctx;
}

export function getTimelineContext(): TimelineContext {
  return getContext(TIMELINE_CONTEXT_KEY);
}
