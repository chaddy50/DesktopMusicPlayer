import { formatTimeDuration } from '@/common/Utilities';
import '@/components/Common/TrackInfo/TrackInfo.css';
import AlbumData from '@/dataObjects/AlbumData';
import TrackData from '@/dataObjects/TrackData';
import { invoke } from '@tauri-apps/api/core';
import { useCallback } from 'react';

interface TrackProps {
	track: TrackData;
	album: AlbumData;
}

function Track(props: TrackProps) {
	const { track, album } = props;

	const playTrack = useCallback(() => {
		invoke('on_track_double_clicked', { track, album });
	}, [track]);

	return (
		<div className='trackContainer' onDoubleClick={playTrack}>
			<span className='trackInfoColumnTrackNumber'>{track.track_number}</span>
			<span className='trackInfoColumn'>{track.name}</span>
			<span className='trackInfoColumn'>{track.artist_name}</span>
			<span className='trackInfoColumnDuration'>
				{formatTimeDuration(track.duration_in_seconds)}
			</span>
		</div>
	);
}

export default Track;
