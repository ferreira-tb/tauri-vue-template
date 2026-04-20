import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: ['tsconfig.json', 'app/tsconfig.json'],
  ignores: [],
  features: {
    vue: true,
  },
  overrides: {},
});
