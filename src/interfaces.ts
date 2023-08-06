export interface Ingredient {
    name: string,
    multiplier: number,
    unit: string,
    img: string
}

export interface IngredientMapping extends Ingredient {
    value: number
}

export interface SolvedIngredient {
    property_index: number,
    optimal_value: number
}