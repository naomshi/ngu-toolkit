import { useState } from "react";
import PitReward from "../PitReward/PitReward";
import { PitRewardMapping } from "../../interfaces";
import { invoke } from "@tauri-apps/api/tauri";

const MoneyPit = () => {
    const [pitRewards, setPitRewards] = useState<PitRewardMapping[]>([]);

    const getPitRewards = async () => {
        const pitRewards: PitRewardMapping[] = await invoke("get_pit_rewards");

        setPitRewards(pitRewards);
    }

    return (
        <>
            <h1 className="text-xl font-bold text-center tracking-wide" style={{ fontFamily: "TheBoldFont, sans-serif" }}>Money Pit</h1>
            <div className="mb-2">Predict which rewards you'll get from the money pit.</div>
            <button onClick={getPitRewards} className="bg-green-600 active:bg-green-500 text-white font-bold py-2 px-4 rounded">
                Get Rewards
            </button>
            <p className="mt-4 text-md font-bold">Upcoming Rewards</p>
            <ul className="mt-2">
                {pitRewards.map((reward: PitRewardMapping) => (
                    <li>
                        <PitReward description={reward.description} img={reward.img} />
                    </li>
                ))}
            </ul>
        </>
    )
}

export default MoneyPit;