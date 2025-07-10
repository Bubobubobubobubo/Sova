import React from 'react';
import { LineState } from '../stores/sceneStore';
import { useColorContext } from '../context/ColorContext';

export interface GridCellProps {
  line: LineState;
  frameIndex: number;
  isSelected: boolean;
  isPlaying: boolean;
  progression?: number; // 0.0 to 1.0
  width: number;
  height: number;
  onClick: () => void;
  onDoubleClick: () => void;
}

export const GridCell: React.FC<GridCellProps> = ({
  line,
  frameIndex,
  isSelected,
  isPlaying,
  progression,
  width,
  height,
  onClick,
  onDoubleClick
}) => {
  const { palette } = useColorContext();
  const frameValue = line.frames[frameIndex];
  const isEnabled = line.enabledFrames[frameIndex];
  const frameName = line.frameNames[frameIndex];
  const repetitions = line.frameRepetitions[frameIndex] || 1;

  const getCellStyle = () => {
    if (isSelected) {
      return {
        backgroundColor: palette.primary,
        color: palette.background
      };
    }
    if (isPlaying) {
      return {
        backgroundColor: palette.warning,
        color: palette.background
      };
    }
    if (isEnabled) {
      return {
        backgroundColor: palette.success,
        color: palette.background
      };
    }
    return {
      backgroundColor: palette.surface,
      color: palette.muted
    };
  };

  const getProgressionStyle = () => {
    if (progression !== undefined && progression > 0) {
      return {
        background: `linear-gradient(to right, 
          rgba(255, 255, 255, 0.3) 0%, 
          rgba(255, 255, 255, 0.3) ${progression * 100}%, 
          transparent ${progression * 100}%)`
      };
    }
    return {};
  };

  const cellStyle = getCellStyle();

  return (
    <div
      className="relative border cursor-pointer flex flex-col justify-between p-1 text-xs hover:opacity-80 transition-opacity"
      style={{
        width: `${width}px`,
        height: `${height}px`,
        backgroundColor: cellStyle.backgroundColor,
        color: cellStyle.color,
        borderColor: palette.border,
        ...getProgressionStyle()
      }}
      onClick={onClick}
      onDoubleClick={onDoubleClick}
    >
      {/* Top row - play marker and name */}
      <div className="flex justify-between items-start h-4 overflow-hidden">
        <span className="text-xs opacity-60">
          {isPlaying ? '▶' : ' '}
        </span>
        <span className="text-xs font-medium truncate max-w-[60px]">
          {frameName || ''}
        </span>
      </div>

      {/* Bottom row - duration and repetitions */}
      <div className="flex justify-end items-end h-4">
        <span 
          className="text-xs px-1"
          style={{
            backgroundColor: palette.background,
            color: palette.text
          }}
        >
          {repetitions > 1 ? `${frameValue.toFixed(1)} × ${repetitions}` : frameValue.toFixed(1)}
        </span>
      </div>

      {/* Progress bar overlay */}
      {progression !== undefined && progression > 0 && (
        <div 
          className="absolute bottom-0 left-0 h-1"
          style={{ 
            width: `${progression * 100}%`,
            backgroundColor: palette.error
          }}
        />
      )}
    </div>
  );
};