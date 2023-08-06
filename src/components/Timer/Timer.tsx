import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { TimerName } from '../../enums';

type NavbarProps = {
  timerName: TimerName,
  name: string,
  description: string,
  img: string
}

const Timer: React.FC<NavbarProps> = ({ timerName, name, description, img }) => {
    const [checked, setChecked] = useState(false);
  
    const handleChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
      const isChecked = e.target.checked;
      setChecked(isChecked);
  
      if (isChecked) {
        await invoke("enable_timer", { timerName: TimerName[timerName] });
      } else {
        await invoke("disable_timer", { timerName: TimerName[timerName] });
      }
    };
  
    return (
      <div className="flex justify-between items-center mb-2">
        <div className="flex items-center">
          <img src={"/img/assets/" + img} className="mr-2 w-12"/>
          <div className="text-left mr-6">
            <p className="text-md font-bold">{name}</p>
            <p>{description}</p>
          </div>
        </div>
        <input 
          id="default-checkbox" 
          type="checkbox" 
          checked={checked} 
          onChange={handleChange} 
          className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
        />
      </div>
    )
  }

export default Timer;