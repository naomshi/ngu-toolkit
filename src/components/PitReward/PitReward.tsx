import React from 'react';

type PitRewardProps = {
    description: string,
    img: string
}

const PitReward: React.FC<PitRewardProps> = ({ description, img }) => {
    return (
        <div className="flex justify-between items-center mb-2">
            <div className="flex items-center">
                <img src={"/img/assets/" + img} className="mr-2 w-12" />
                <p className="text-md">{description}</p>
            </div>
        </div>
    )
}

export default PitReward;