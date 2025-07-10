import { atom, map } from 'nanostores';

export interface SceneState {
  length: number;
  lines: LineState[];
}

export interface LineState {
  frames: number[];              // Frame durations in beats
  enabledFrames: boolean[];      // Which frames are active
  frameNames: (string | null)[];  // Optional frame names
  frameRepetitions: number[];    // Repetition counts
  speedFactor: number;           // Playback speed multiplier
  customLength?: number;         // Override total duration
  index: number;                 // Position within parent scene
  startFrame?: number;           // Optional playback start
  endFrame?: number;             // Optional playback end
}

export interface GridSelection {
  start: [number, number];       // [row, col]
  end: [number, number];         // [row, col]
}

export interface GridState {
  selection: GridSelection;
  scrollOffset: number;
  showHelp: boolean;
  maxFrames: number;
  visibleHeight: number;
}

// Scene store - core musical data
export const sceneStore = atom<SceneState | null>(null);

// Grid UI state
export const gridStore = map<GridState>({
  selection: { start: [0, 0], end: [0, 0] },
  scrollOffset: 0,
  showHelp: false,
  maxFrames: 0,
  visibleHeight: 0
});

// Grid progression cache for performance
export const progressionCache = atom<Map<string, number>>(new Map());

// Helper functions
export const updateScene = (scene: SceneState) => {
  sceneStore.set(scene);
  
  // Update grid state based on scene
  const maxFrames = Math.max(...scene.lines.map(line => line.frames.length));
  gridStore.setKey('maxFrames', maxFrames);
};

export const updateGridSelection = (selection: GridSelection) => {
  gridStore.setKey('selection', selection);
};

export const updateGridScrollOffset = (offset: number) => {
  gridStore.setKey('scrollOffset', offset);
};

export const createEmptyScene = (): SceneState => ({
  length: 16,
  lines: [
    {
      frames: [1.0, 1.0, 1.0, 1.0],
      enabledFrames: [true, false, true, false],
      frameNames: ['kick', null, 'snare', null],
      frameRepetitions: [1, 1, 1, 1],
      speedFactor: 1.0,
      index: 0
    }
  ]
});

export const getGridSelectionBounds = (selection: GridSelection): [[number, number], [number, number]] => {
  const [startRow, startCol] = selection.start;
  const [endRow, endCol] = selection.end;
  
  return [
    [Math.min(startRow, endRow), Math.min(startCol, endCol)],
    [Math.max(startRow, endRow), Math.max(startCol, endCol)]
  ];
};

export const isGridSelectionSingle = (selection: GridSelection): boolean => {
  return selection.start[0] === selection.end[0] && selection.start[1] === selection.end[1];
};