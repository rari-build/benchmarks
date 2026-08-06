export interface TodoItem {
  id: number
  text: string
  completed: boolean
}

export function getTodosList(): TodoItem[] {
  return [
    { id: 1, text: '🥛 Buy milk', completed: true },
    { id: 2, text: '🍞 Get fresh bread', completed: true },
    { id: 3, text: '🥕 Pick up carrots', completed: true },
    { id: 4, text: '🍎 Red apples (6 pack)', completed: true },
    { id: 5, text: '🥩 Ground beef (1 lb)', completed: false },
    { id: 6, text: '🧀 Cheddar cheese', completed: false },
    { id: 7, text: '🍋 Lemons for cooking', completed: false },
    { id: 8, text: '🥬 Fresh lettuce', completed: false },
    { id: 9, text: '🍝 Pasta for dinner', completed: false },
    { id: 10, text: '☕ Coffee beans', completed: false },
  ]
}

export function add(a: number, b: number): number {
  return a + b
}
