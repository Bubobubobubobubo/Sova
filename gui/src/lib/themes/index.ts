export type { Theme, FontOption } from './types';
export { fontFamilies, combineWithSystemFonts } from './types';
export { createHighlightStyle } from './utils';

import type { Theme } from './types';
import { light } from './definitions/light';
import { blue } from './definitions/blue';
import { monokai } from './definitions/monokai';
import { monolight } from './definitions/monolight';
import { monodark } from './definitions/monodark';
import { hacker } from './definitions/hacker';
import { dracula } from './definitions/dracula';
import { nord } from './definitions/nord';
import { darcula } from './definitions/darcula';
import { bluescreen } from './definitions/bluescreen';
import { gruvbox } from './definitions/gruvbox';
import { solarizeddark } from './definitions/solarizeddark';
import { solarizedlight } from './definitions/solarizedlight';
import { tokyonight } from './definitions/tokyonight';
import { catppuccin } from './definitions/catppuccin';
import { ayumirage } from './definitions/ayumirage';
import { onedarkpro } from './definitions/onedarkpro';
import { nightowl } from './definitions/nightowl';
import { materialdarker } from './definitions/materialdarker';
import { georges } from './definitions/georges';
import { ghosttyThemes } from './definitions/ghostty/index';

const originalThemes = {
  light,
  blue,
  monokai,
  monolight,
  monodark,
  hacker,
  dracula,
  nord,
  darcula,
  bluescreen,
  gruvbox,
  solarizeddark,
  solarizedlight,
  tokyonight,
  catppuccin,
  ayumirage,
  onedarkpro,
  nightowl,
  materialdarker,
  georges,
};

export const themes: Record<string, Theme> = {
  ...originalThemes,
  ...ghosttyThemes,
};

export type ThemeName = keyof typeof themes;
