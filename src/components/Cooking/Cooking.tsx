import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import { IngredientMapping, SolvedIngredient } from "../../interfaces"
import { ingredientsMap } from "../../data/CookingIngredients"

export default function Cooking() {
  const [ingredientMappings, setIngredientMappings] = useState<IngredientMapping[]>([]);

  const getIngredientsMap = async () => {
    const solvedIngredients: SolvedIngredient[] = await invoke("solve_cooking");

    const map = solvedIngredients.map((solvedIngredient: SolvedIngredient) => {
      const target = ingredientsMap[solvedIngredient.property_index];

      const ingredientMapping: IngredientMapping = {
        name: target.name,
        multiplier: target.multiplier,
        value: solvedIngredient.optimal_value,
        unit: target.unit,
        img: target.img
      }

      return ingredientMapping;
    });

    setIngredientMappings(map);
  }

  return (
    <>
      <h1 className="text-xl font-bold text-center tracking-wide" style={{fontFamily: "TheBoldFont, sans-serif"}}>Cooking</h1>
      <div className="mb-2">Press the button below to calculate the best ingredients combination.</div>
      <button onClick={getIngredientsMap} className="bg-green-600 active:bg-green-500 text-white font-bold py-2 px-4 rounded">
        Solve!
      </button>
      <div className="grid grid-cols-2 gap-4 mt-4">
        {ingredientMappings.map((ingredientMapping, idx) => (
          <div key={idx} className="flex items-center space-x-4">
            <div className="flex-shrink-0">
              <div className="h-16 w-auto">
                <img src={"/img/assets/" + ingredientMapping.img} alt={ingredientMapping.name} className="h-16 w-16 object-contain" />
              </div>
            </div>
            <div className="text-left">
                <div>{ingredientMapping.name}</div>
                <div>{ingredientMapping.multiplier * ingredientMapping.value} {ingredientMapping.unit}</div>
            </div>
          </div>
        ))}
      </div>
    </>
  );
}
