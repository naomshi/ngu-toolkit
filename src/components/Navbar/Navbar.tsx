import { MenuSelection } from "../../enums";
import NavButton from "../NavButton/NavButton";

type NavbarProps = {
  setMenuSelection: React.Dispatch<React.SetStateAction<MenuSelection>>;
};

const Navbar: React.FC<NavbarProps> = ({ setMenuSelection }) => {
  return (
    <div className="bg-gray-800 text-white py-4">
      <h1 className="text-4xl font-bold text-center mb-4 tracking-wide" style={{ fontFamily: "TheBoldFont, sans-serif" }}>NGU Toolkit</h1>
      <div className="flex justify-center space-x-4 text-sm">
        <NavButton setMenuSelection={setMenuSelection} menuSelection={MenuSelection.Notifications} title="Notifications" />
        <NavButton setMenuSelection={setMenuSelection} menuSelection={MenuSelection.Cooking} title="Cooking" />
        <NavButton setMenuSelection={setMenuSelection} menuSelection={MenuSelection.MoneyPit} title="Money Pit" />
      </div>
    </div>
  );
};

export default Navbar;
