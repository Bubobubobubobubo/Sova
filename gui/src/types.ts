export interface GridSelection {
  start: [number, number];
  end: [number, number];
}

export type ActionTiming = 
  | "Immediate"
  | { EndOfLine: number }
  | { AtBeat: number };

// Re-export frame types for convenience
export type { Frame, FramePosition, DraggedFrame, PastedFrameData } from './types/frame';
import type { Frame, PastedFrameData } from './types/frame';

export interface Script {
  content: string;
  lang?: string;
}

export interface Line {
  frames: Frame[];
  speed_factor: number;
  index: number;
  start_frame: number | undefined;
  end_frame: number | undefined;
  custom_length: number | undefined;
}

export interface Scene {
  lines: Line[];
}

export interface DeviceInfo {
  id: number;
  name: string;
  kind: "Midi" | "Osc" | "Log" | "Other";
  is_connected: boolean;
  address: string | undefined;
}

export interface CompilationError {
  lang: string;
  info: string;
  from: number | undefined;
  to: number | undefined;
}

export interface Snapshot {
  scene: Scene;
  tempo: number;
  beat: number;
  micros: number;
  quantum: number;
}

export type VariableValue = 
  | { Integer: number }
  | { Float: number }
  | { Bool: boolean }
  | { Str: string }
  | [number, number, number] // Decimal as tuple [sign, numerator, denominator]
  | any; // Catch-all for complex types like Dur, Func, Map

export type SchedulerMessage = 
  | "Play"
  | "Stop"
  | "Pause"
  | "Reset"
  | { Seek: number };

export type ClientMessage = 
  | { SchedulerControl: SchedulerMessage }
  | { SetTempo: [number, ActionTiming] }
  | { SetName: string }
  | "GetScene"
  | { SetScene: [Scene, ActionTiming] }
  | "GetLine"
  | { SetLines: [[number, Line][], ActionTiming] }
  | { ConfigureLines: [[number, Line][], ActionTiming] }
  | "GetClock"
  | "GetPeers"
  | { Chat: string }
  | { UpdateLineFrames: [number, number[], ActionTiming] }
  | { InsertFrame: [number, number, number, ActionTiming] }
  | { RemoveFrame: [number, number, ActionTiming] }
  | { SetLineStartFrame: [number, number | null, ActionTiming] }
  | { SetLineEndFrame: [number, number | null, ActionTiming] }
  | "GetSnapshot"
  | { UpdateGridSelection: GridSelection }
  | { StartedEditingFrame: [number, number] }
  | { StoppedEditingFrame: [number, number] }
  | { TransportStart: ActionTiming }
  | { TransportStop: ActionTiming }
  | "RequestDeviceList"
  | { ConnectMidiDeviceById: number }
  | { DisconnectMidiDeviceById: number }
  | { ConnectMidiDeviceByName: string }
  | { DisconnectMidiDeviceByName: string }
  | { CreateVirtualMidiOutput: string }
  | { AssignDeviceToSlot: [number, string] }
  | { UnassignDeviceFromSlot: number }
  | { CreateOscDevice: [string, string, number] }
  | { RemoveOscDevice: string };

export type ServerMessage = 
  | { Hello: {
      username: string;
      scene: Scene;
      devices: DeviceInfo[];
      peers: string[];
      link_state: [number, number, number, number, boolean];
      is_playing: boolean;
      available_compilers: string[];
      syntax_definitions: Record<string, string>;
    } }
  | { ConnectionRefused: string }
  | "Success"
  | { InternalError: string }
  | { SceneValue: Scene }
  | { ScriptContent: {
      line_idx: number;
      frame_idx: number;
      content: string;
    } }
  | { ScriptCompiled: {
      line_idx: number;
      frame_idx: number;
    } }
  | { CompilationErrorOccurred: CompilationError }
  | "TransportStarted"
  | "TransportStopped"
  | { ClockState: [number, number, number, number] }
  | { FramePosition: [number, number, number][] }
  | { DeviceList: DeviceInfo[] }
  | { PeersUpdated: string[] }
  | { PeerGridSelectionUpdate: [string, GridSelection] }
  | { PeerStartedEditing: [string, number, number] }
  | { PeerStoppedEditing: [string, number, number] }
  | { Chat: string }
  | { LogString: string }
  | { Snapshot: Snapshot }
  | { GlobalVariablesUpdate: Record<string, VariableValue> };

export interface BuboClient {
  connect: (ip: string, port: number) => Promise<void>;
  disconnect: () => Promise<void>;
  sendMessage: (message: ClientMessage) => Promise<void>;
  getMessages: () => Promise<ServerMessage[]>;
  isConnected: () => Promise<boolean>;
  onMessage: (callback: (message: ServerMessage) => void) => void;
}