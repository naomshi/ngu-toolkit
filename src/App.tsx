import { useState } from "react";
import Navbar from "./components/Navbar/Navbar";
import { MenuSelection } from "./enums";
import Cooking from "./components/Cooking/Cooking";
import Notifications from "./components/Notifications/Notifications";
import MoneyPit from "./components/MoneyPit/MoneyPit";

function App() {
  const [menuSelection, setMenuSelection] = useState<MenuSelection>(MenuSelection.Notifications);

  const renderMenuSelection = () => {
    switch(menuSelection) {
      case MenuSelection.Cooking:
        return <Cooking/>
      case MenuSelection.Notifications:
        return <Notifications/>
      case MenuSelection.MoneyPit:
        return <MoneyPit/>
    }
  }

  return (
    <div className="bg-gray-600 w-screen h-screen text-gray-200">
      <Navbar setMenuSelection={setMenuSelection}/>
      <div className="mt-2 flex flex-col items-center">
      {renderMenuSelection()}
      </div>
      
    </div>
  );
}

export default App;
