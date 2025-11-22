import { Script } from "../types";

export interface Frame {
  duration: number;
  enabled: boolean;
  name: string | null;
  script: Script;
  repetitions: number;
}

export interface FramePosition {
  lineIdx: number;
  frameIdx: number;
}

export interface DraggedFrame extends Frame {
  position: FramePosition;
}

export interface PastedFrameData { // TODO: tout faire en frame ?
  length: number;
  is_enabled: boolean;
  script_content: string;
  name: string | undefined;
  repetitions: number | undefined;
}

// Helper to convert Frame to PastedFrameData
export function frameTopastedData(frame: Frame): PastedFrameData {
  return {
    length: frame.duration,
    is_enabled: frame.enabled,
    script_content: frame.script?.content || "",
    name: frame.name || undefined,
    repetitions: frame.repetitions || undefined,
  };
}

// Helper to convert PastedFrameData to Frame
export function pastedDataToFrame(data: PastedFrameData): Frame {
  return {
    duration: data.length,
    enabled: data.is_enabled,
    name: data.name || null,
    script: { content: data.script_content },
    repetitions: data.repetitions || 1,
  };
}

export function defaultFrame(): Frame {
  return {
    duration: 1.0,
    enabled: true,
    name: null,
    script: { content: '', lang: 'bali' },
    repetitions: 1
  };
}