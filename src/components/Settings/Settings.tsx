import { invoke } from '@tauri-apps/api/core';
import { useCallback, useState } from 'react';
import Directories from './Directories';
import './Settings.css';

const Settings = () => {
	const [directories, setDirectories] = useState<string[]>([]);

	const saveSettings = useCallback(() => {
		invoke('save_settings', { directories });
	}, [directories]);

	const rebuildMusicDatabase = useCallback(() => {
		invoke('rebuild_music_database');
	}, []);

	return (
		<div className='settingsContainer'>
			<h1>Settings</h1>
			<Directories directories={directories} setDirectories={setDirectories} />
			<br />
			<button onClick={saveSettings}>Save Settings</button>
			<button onClick={rebuildMusicDatabase}>Rebuild Database</button>
		</div>
	);
};

export default Settings;
