import SettingData from '@/dataObjects/SettingData';
import SettingsStore from '@/state/SettingsStore';
import CreateNewFolderIcon from '@mui/icons-material/CreateNewFolder';
import { open } from '@tauri-apps/plugin-dialog';
import { observer } from 'mobx-react';
import { Dispatch, SetStateAction, useEffect } from 'react';
import DirectoryChooser from './DirectoryChooser';

interface DirectoriesProps {
	directories: string[];
	setDirectories: Dispatch<SetStateAction<string[]>>;
}

const Directories = observer((props: DirectoriesProps) => {
	const { directories, setDirectories } = props;

	useEffect(() => {
		const settings = SettingsStore.settings;

		const directoriesSettings = settings.filter(
			(setting: SettingData) => setting.key === 'directories'
		);

		let directories: string[] = [];
		for (const directorySetting of directoriesSettings) {
			directories.push(directorySetting.value);
		}
		setDirectories(directories);
	}, []);

	const addNewDirectory = async () => {
		const folder = await open({
			multiple: false,
			directory: true,
			defaultPath: '/home/nathan/Music',
		});
		if (folder) {
			setDirectories((previousDirectories) => [...previousDirectories, folder]);
		}
	};

	const changeDirectory = async (directoryIndex: number) => {
		const oldDirectory = directories[directoryIndex];

		const newDirectory = await open({
			multiple: false,
			directory: true,
			defaultPath: oldDirectory,
		});

		if (newDirectory) {
			const newDirectories = [...directories];
			newDirectories[directoryIndex] = newDirectory;
			setDirectories(newDirectories);
		}
	};

	return (
		<div className='directoryListContainer'>
			<h2>Folders</h2>
			{directories.map((directory, directoryIndex) => {
				return (
					<DirectoryChooser
						key={directory + directoryIndex}
						directory={directory}
						chooseFolder={() => changeDirectory(directoryIndex)}
					/>
				);
			})}
			<button className='button' onClick={addNewDirectory}>
				<CreateNewFolderIcon />
			</button>
		</div>
	);
});

export default Directories;
