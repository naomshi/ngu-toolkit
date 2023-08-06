import { TimerName } from "../../enums";
import Timer from "../Timer/Timer";

const Notifications = () => {
    return (
        <>
            <ul className="mt-2">
                <li>
                    <Timer timerName={TimerName.Adventure} name="Adventure Idle" description="Notifies you whenever you enter the safe zone in adventure mode." img="adventure.png" />
                    <Timer timerName={TimerName.Quest} name="Questing" description="Notifies you when you have enough items to complete a quest." img="quest.png" />
                </li>
            </ul>
        </>
    )
}

export default Notifications;