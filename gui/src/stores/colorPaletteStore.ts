import { persistentAtom } from '@nanostores/persistent';
import type { ThemeMode } from '../hooks/useMaterialPalette';

export interface ColorPaletteSettings {
  hueRotation: number;
  themeMode: ThemeMode;
  saturation: number;
  brightness: number;
  basePrimary: number;
  baseSecondary: number;
}

const defaultSettings: ColorPaletteSettings = {
  hueRotation: 0,
  themeMode: 'light',
  saturation: 75,
  brightness: 50,
  basePrimary: Math.floor(Math.random() * 360),
  baseSecondary: (() => {
    const primary = Math.floor(Math.random() * 360);
    const offset = 120 + Math.floor(Math.random() * 120);
    return (primary + offset) % 360;
  })(),
};

export const $colorPaletteSettings = persistentAtom<ColorPaletteSettings>(
  'colorPaletteSettings',
  defaultSettings,
  {
    encode: JSON.stringify,
    decode: JSON.parse,
  }
);

export const updateHueRotation = (value: number) => {
  $colorPaletteSettings.set({
    ...$colorPaletteSettings.get(),
    hueRotation: value,
  });
};

export const updateThemeMode = (mode: ThemeMode) => {
  $colorPaletteSettings.set({
    ...$colorPaletteSettings.get(),
    themeMode: mode,
  });
};

export const updateSaturation = (value: number) => {
  $colorPaletteSettings.set({
    ...$colorPaletteSettings.get(),
    saturation: value,
  });
};

export const updateBrightness = (value: number) => {
  $colorPaletteSettings.set({
    ...$colorPaletteSettings.get(),
    brightness: value,
  });
};

export const regenerateBaseColors = () => {
  const newPrimary = Math.floor(Math.random() * 360);
  const offset = 120 + Math.floor(Math.random() * 120);
  const newSecondary = (newPrimary + offset) % 360;
  
  $colorPaletteSettings.set({
    ...$colorPaletteSettings.get(),
    basePrimary: newPrimary,
    baseSecondary: newSecondary,
    hueRotation: 0,
    saturation: 75,
    brightness: 50,
  });
};

export const toggleTheme = () => {
  const current = $colorPaletteSettings.get();
  updateThemeMode(current.themeMode === 'light' ? 'dark' : 'light');
};