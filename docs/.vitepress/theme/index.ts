import DefaultTheme from 'vitepress/theme';
import type { Theme } from 'vitepress';
import './custom.css';

/**
 * Locus theme. Dark by default. System fonts and files in this package only —
 * no remote fonts, scripts, or images.
 */
const theme: Theme = {
  extends: DefaultTheme,
};

export default theme;
