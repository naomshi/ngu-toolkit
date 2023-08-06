import { TimerName } from "../../enums";
import Timer from "../Timer/Timer";

const Notifications = () => {
    return (
        <>
            <h1 className="text-xl font-bold text-center tracking-wide" style={{fontFamily: "TheBoldFont, sans-serif"}}>Notifications</h1>
            <div className="mb-2">Spawns desktop notifications based on events happening in-game.</div>
            <ul className="mt-2">
                <li>
                    <Timer timerName={TimerName.Adventure} name="Adventure Idle" description="Triggers when you enter the safe zone in adventure mode." img="adventure.png" />
                    <Timer timerName={TimerName.Quest} name="Questing" description="Triggers when you have enough items to complete a quest." img="quest.png" />
                    <Timer timerName={TimerName.Muffin} name="Muffin" description="Triggers if you're trying to rebirth without a muffin active." img="muffin.png" />
                    <Timer timerName={TimerName.Inventory} name="Inventory" description="Triggers when your inventory is full." img="inventory.png" />
                </li>
            </ul>
        </>
    )
}

export default Notifications;