import NowPlayingStore from '@/state/NowPlayingStore';
import { mockTrack1, mockTrack2 } from '@/tests/MockObjects';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import NowPlaying from './NowPlaying';

describe('NowPlaying', () => {
	it('renders with no playing tracks', () => {
		NowPlayingStore.update({
			playing_tracks: [],
			playing_track_index: -1,
			is_paused: false,
			is_playing: false,
		});

		render(<NowPlaying />);

		expect(screen.getByTestId('nowPlayingContainer')).toBeDefined();
		expect(screen.queryByTestId('playingTracksContainer')).toBeNull();
	});

	it('renders with playing tracks', () => {
		NowPlayingStore.update({
			playing_tracks: [mockTrack1, mockTrack2],
			playing_track_index: 0,
			is_paused: false,
			is_playing: false,
		});

		render(<NowPlaying />);

		expect(screen.getByTestId('nowPlayingContainer')).toBeDefined();
		expect(
			screen.getByTestId('trackInfo' + mockTrack1.file_path)
		).toBeDefined();
		expect(
			screen.getByTestId('trackInfo' + mockTrack2.file_path)
		).toBeDefined();
	});
});
