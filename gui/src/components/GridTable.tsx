import React, { useEffect, useRef } from 'react';
import { useStore } from '@nanostores/react';
import { GridCell } from './GridCell';
import { sceneStore, gridStore, updateGridSelection } from '../stores/sceneStore';
import { useColorContext } from '../context/ColorContext';

export interface GridTableProps {
  cellWidth: number;
  cellHeight: number;
  containerWidth: number;
  containerHeight: number;
}

export const GridTable: React.FC<GridTableProps> = ({
  cellWidth,
  cellHeight,
  containerWidth,
  containerHeight
}) => {
  const scene = useStore(sceneStore);
  const grid = useStore(gridStore);
  const { palette } = useColorContext();
  const containerRef = useRef<HTMLDivElement>(null);

  if (!scene || scene.lines.length === 0) {
    return (
      <div 
        className="flex items-center justify-center h-full"
        style={{ color: palette.muted }}
      >
        No scene loaded
      </div>
    );
  }

  const maxFrames = Math.max(...scene.lines.map(line => line.frames.length));
  const visibleRows = Math.floor(containerHeight / cellHeight);
  const visibleCols = Math.floor(containerWidth / cellWidth);

  const handleCellClick = (rowIndex: number, colIndex: number) => {
    updateGridSelection({
      start: [rowIndex, colIndex],
      end: [rowIndex, colIndex]
    });
  };

  const handleCellDoubleClick = (rowIndex: number, colIndex: number) => {
    // TODO: Open frame editor
    console.log('Edit frame:', rowIndex, colIndex);
  };

  const isSelected = (rowIndex: number, colIndex: number): boolean => {
    const [[minRow, minCol], [maxRow, maxCol]] = [
      [Math.min(grid.selection.start[0], grid.selection.end[0]), Math.min(grid.selection.start[1], grid.selection.end[1])],
      [Math.max(grid.selection.start[0], grid.selection.end[0]), Math.max(grid.selection.start[1], grid.selection.end[1])]
    ];
    return rowIndex >= minRow && rowIndex <= maxRow && colIndex >= minCol && colIndex <= maxCol;
  };

  const renderGrid = () => {
    const cells = [];
    
    // Render visible rows
    for (let row = grid.scrollOffset; row < Math.min(grid.scrollOffset + visibleRows, maxFrames); row++) {
      const rowCells = [];
      
      // Render each column (line)
      for (let col = 0; col < Math.min(scene.lines.length, visibleCols); col++) {
        const line = scene.lines[col];
        const hasFrame = row < line.frames.length;
        
        if (hasFrame) {
          rowCells.push(
            <GridCell
              key={`${row}-${col}`}
              line={line}
              frameIndex={row}
              isSelected={isSelected(row, col)}
              isPlaying={false} // TODO: Connect to playback state
              progression={undefined} // TODO: Connect to progression
              width={cellWidth}
              height={cellHeight}
              onClick={() => handleCellClick(row, col)}
              onDoubleClick={() => handleCellDoubleClick(row, col)}
            />
          );
        } else {
          // Empty cell placeholder
          rowCells.push(
            <div
              key={`${row}-${col}`}
              className="border"
              style={{ 
                width: cellWidth, 
                height: cellHeight,
                backgroundColor: palette.background,
                borderColor: palette.border
              }}
            />
          );
        }
      }
      
      cells.push(
        <div key={row} className="flex">
          {rowCells}
        </div>
      );
    }
    
    return cells;
  };

  return (
    <div
      ref={containerRef}
      className="overflow-hidden border"
      style={{ 
        width: containerWidth, 
        height: containerHeight,
        backgroundColor: palette.background,
        borderColor: palette.border
      }}
    >
      {/* Column headers */}
      <div 
        className="flex border-b"
        style={{
          backgroundColor: palette.surface,
          borderColor: palette.border
        }}
      >
        {scene.lines.slice(0, visibleCols).map((line, index) => (
          <div
            key={index}
            className="flex items-center justify-center border-r text-xs font-medium"
            style={{ 
              width: cellWidth, 
              height: 24,
              color: palette.text,
              borderColor: palette.border
            }}
          >
            Line {index}
          </div>
        ))}
      </div>

      {/* Grid body */}
      <div className="flex flex-col">
        {renderGrid()}
      </div>
    </div>
  );
};