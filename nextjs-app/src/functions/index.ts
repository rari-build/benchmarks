'use server'

/**
 * Get the list of grocery items
 */
export async function getTodosList() {
  const groceries = [
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

  return groceries
}

/**
 * Simple server function for testing
 */
export async function add(a: number, b: number): Promise<number> {
  // Simulate some async work
  await new Promise(resolve => setTimeout(resolve, 1))
  return a + b
}

/**
 * Another test function for demonstration
 */
export async function multiply(a: number, b: number): Promise<number> {
  await new Promise(resolve => setTimeout(resolve, 1))
  return a * b
}
