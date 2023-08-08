import { MenuSelection } from "../../enums";

type NavButtonProps = {
    setMenuSelection: React.Dispatch<React.SetStateAction<MenuSelection>>,
    menuSelection: MenuSelection,
    title: string
};

const NavButton: React.FC<NavButtonProps> = ({ setMenuSelection, menuSelection, title }) => {
    return (
        <button onClick={() => setMenuSelection(menuSelection)} className="bg-blue-700 hover:bg-blue-700 active:bg-blue-800 text-white font-bold py-2 px-4 rounded">
            { title }
        </button>
    )
};

export default NavButton;
