import { formatTimeDuration } from '@/common/Utilities';
import '@/components/MusicBrowser/RightSidebar/NowPlaying/TrackInfo/TrackInfo.css';
import AlbumData from '@/dataObjects/AlbumData';
import TrackData from '@/dataObjects/TrackData';
import NowPlayingStore from '@/state/NowPlayingStore';
import { invoke } from '@tauri-apps/api/core';
import { observer } from 'mobx-react';
import { useCallback } from 'react';

interface TrackProps {
	track: TrackData;
	album: AlbumData;
}

const Track = observer((props: TrackProps) => {
	const { track, album } = props;
	const playingTrack = NowPlayingStore.playingTrack;

	const playTrack = useCallback(() => {
		invoke('on_track_double_clicked', { track, album });
	}, [track]);

	let containerClassName = 'trackContainer';
	if (playingTrack?.file_path === track.file_path) {
		containerClassName += ' trackContainerSelected';
	}

	return (
		<div
			data-testid={'trackContainer' + track.file_path}
			className={containerClassName}
			onDoubleClick={playTrack}
		>
			<span className='trackInfoColumnTrackNumber'>{track.track_number}</span>
			<span className='trackInfoColumn'>{track.name}</span>
			<span className='trackInfoColumn'>{track.artist_name}</span>
			<span className='trackInfoColumnDuration'>
				{formatTimeDuration(track.duration_in_seconds)}
			</span>
		</div>
	);
});

export default Track;
