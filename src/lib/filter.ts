export const FILTERS = [
  'Point',
  'Hermite',
  'Cubic',
  'Box',
  'Gaussian',
  'Catrom',
  'Triangle',
  'Quadratic',
  'Mitchell',
  'CubicSpline',
]

export type Filter = (typeof FILTERS)[number]
