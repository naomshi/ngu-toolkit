import { MenuSelection } from "../../enums";

type NavbarProps = {
  setMenuSelection: React.Dispatch<React.SetStateAction<MenuSelection>>;
};

const Navbar: React.FC<NavbarProps> = ({ setMenuSelection }) => {
  return (
    <div className="bg-gray-800 text-white py-4">
      <h1 className="text-4xl font-bold text-center mb-4 tracking-wide" style={{ fontFamily: "TheBoldFont, sans-serif" }}>NGU Toolkit</h1>
      <div className="flex justify-center space-x-4 text-sm">
        <button onClick={() => setMenuSelection(MenuSelection.Notifications)} className="bg-blue-700 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
          Notifications
        </button>
        <button onClick={() => setMenuSelection(MenuSelection.Cooking)} className="bg-blue-700 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
          Cooking
        </button>
      </div>
    </div>
  );
};

export default Navbar;
