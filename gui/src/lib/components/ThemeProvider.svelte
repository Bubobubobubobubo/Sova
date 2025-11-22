<script lang="ts">
  import { currentTheme } from '$lib/stores/themeStore';
  import type { Snippet } from 'svelte';

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();

  function toKebabCase(str: string): string {
    return str.replace(/([a-z])([A-Z])/g, '$1-$2').toLowerCase();
  }

  function flattenTheme(theme: any): Record<string, string> {
    const result: Record<string, string> = {};

    for (const [section, values] of Object.entries(theme)) {
      if (section === 'name' || typeof values !== 'object') continue;

      for (const [key, value] of Object.entries(values as Record<string, string>)) {
        result[`--${section}-${toKebabCase(key)}`] = value;
      }
    }

    return result;
  }

  const themeVars = $derived(flattenTheme($currentTheme));

  const styleString = $derived(
    Object.entries(themeVars)
      .map(([key, value]) => `${key}: ${value}`)
      .join('; ')
  );
</script>

<div class="theme-provider" style={styleString}>
  {@render children()}
</div>

<style>
  .theme-provider {
    display: contents;
  }
</style>
