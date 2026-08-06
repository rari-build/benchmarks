import type { OxlintConfig } from 'vite-plus/lint'
import { fmt as rariFmt, lint as rariLint } from '@rari/lint/vite'

const extraIgnores = ['**/.next/**', '**/next-env.d.ts', '**/results/**']

const readonlyParameterAllows: Array<
  string | { from: 'lib'; name: string[] } | { from: 'package'; name: string[]; package: string }
> = [
  {
    from: 'lib',
    name: ['URL', 'AbortSignal', 'Error', 'TypeError', 'RegExp', 'Date', 'Uint8Array'],
  },
  {
    from: 'package',
    name: [
      'ReactElement',
      'ReactPortal',
      'ReactNode',
      'SyntheticEvent',
      'MouseEvent',
      'CSSProperties',
      'SVGProps',
    ],
    package: 'react',
  },
  { from: 'package', name: ['PageProps', 'LayoutProps', 'Metadata'], package: 'rari' },
  { from: 'package', name: ['Metadata'], package: 'next' },
]

export const fmt = {
  ...rariFmt,
  ignorePatterns: [...(rariFmt.ignorePatterns ?? []), ...extraIgnores],
}

export const lint: OxlintConfig = {
  ...rariLint,
  ignorePatterns: [...(rariLint.ignorePatterns ?? []), ...extraIgnores],
  rules: {
    ...rariLint.rules,
    'typescript/prefer-readonly-parameter-types': [
      'error',
      {
        ignoreInferredTypes: true,
        treatMethodsAsReadonly: true,
        allow: readonlyParameterAllows,
      },
    ],
  },
}
