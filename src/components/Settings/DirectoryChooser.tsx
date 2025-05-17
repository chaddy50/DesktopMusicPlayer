import FolderIcon from '@mui/icons-material/Folder';

interface DirectoryChooserProps {
	directory: string;
	chooseFolder: () => void;
}

const DirectoryChooser = (props: DirectoryChooserProps) => {
	const { directory, chooseFolder } = props;

	return (
		<div className='directoryChooserContainer'>
			<button className='button' onClick={chooseFolder} title={directory}>
				<FolderIcon className='chooseDirectoryIcon' />
				<span>{getFolderName(directory)}</span>
			</button>
		</div>
	);
};

function getFolderName(directory: string): string {
	const pieces = directory.split('/');
	console.log(pieces);
	return pieces[pieces.length - 1];
}

export default DirectoryChooser;
