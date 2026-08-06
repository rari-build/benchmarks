import rariLint from '@rari/lint/eslint'

export default [
  ...rariLint,
  {
    ignores: ['**/.next/**', '**/next-env.d.ts', '**/results/**'],
  },
]
