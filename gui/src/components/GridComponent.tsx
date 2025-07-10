import React, { useEffect, useRef, useState } from 'react';
import { useStore } from '@nanostores/react';
import { GridTable } from './GridTable';
import { sceneStore, gridStore, updateScene, createEmptyScene, updateGridSelection } from '../stores/sceneStore';
import { useColorContext } from '../context/ColorContext';

export interface GridComponentProps {
  width: number;
  height: number;
}

export const GridComponent: React.FC<GridComponentProps> = ({
  width,
  height
}) => {
  const scene = useStore(sceneStore);
  const grid = useStore(gridStore);
  const { palette } = useColorContext();
  const [cellWidth] = useState(80);
  const [cellHeight] = useState(60);
  const containerRef = useRef<HTMLDivElement>(null);

  // Initialize with demo data if no scene exists
  useEffect(() => {
    if (!scene) {
      updateScene(createEmptyScene());
    }
  }, [scene]);

  const handleKeyDown = (event: React.KeyboardEvent) => {
    if (!scene) return;

    const { selection } = grid;
    const [currentRow, currentCol] = selection.end;
    const maxFrames = Math.max(...scene.lines.map(line => line.frames.length));
    const maxCols = scene.lines.length;

    let newRow = currentRow;
    let newCol = currentCol;
    let handled = false;

    switch (event.key) {
      case 'ArrowUp':
        newRow = Math.max(0, currentRow - 1);
        handled = true;
        break;
      case 'ArrowDown':
        newRow = Math.min(maxFrames - 1, currentRow + 1);
        handled = true;
        break;
      case 'ArrowLeft':
        newCol = Math.max(0, currentCol - 1);
        handled = true;
        break;
      case 'ArrowRight':
        newCol = Math.min(maxCols - 1, currentCol + 1);
        handled = true;
        break;
      case 'Escape':
        // Reset selection to single cell
        updateGridSelection({
          start: [currentRow, currentCol],
          end: [currentRow, currentCol]
        });
        handled = true;
        break;
    }

    if (handled) {
      event.preventDefault();
      
      // Clamp row to available frames in the selected column
      const line = scene.lines[newCol];
      if (line && newRow >= line.frames.length) {
        newRow = Math.max(0, line.frames.length - 1);
      }

      if (event.shiftKey) {
        // Extend selection
        updateGridSelection({
          start: selection.start,
          end: [newRow, newCol]
        });
      } else {
        // Move cursor
        updateGridSelection({
          start: [newRow, newCol],
          end: [newRow, newCol]
        });
      }
    }
  };

  if (!scene) {
    return (
      <div 
        className="flex items-center justify-center h-full"
        style={{ 
          backgroundColor: palette.background,
          color: palette.muted 
        }}
      >
        Loading...
      </div>
    );
  }

  return (
    <div
      ref={containerRef}
      className="border"
      style={{ 
        width, 
        height,
        backgroundColor: palette.background,
        borderColor: palette.border
      }}
      tabIndex={0}
      onKeyDown={handleKeyDown}
    >
      {/* Grid */}
      <GridTable
        cellWidth={cellWidth}
        cellHeight={cellHeight}
        containerWidth={width}
        containerHeight={height}
      />
    </div>
  );
};