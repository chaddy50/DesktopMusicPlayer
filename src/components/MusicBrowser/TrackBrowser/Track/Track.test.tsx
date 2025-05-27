import NowPlayingStore from '@/state/NowPlayingStore';
import { mockAlbum1, mockTrack1 } from '@/tests/MockObjects';
import { render, screen } from '@testing-library/react';
import { beforeAll, describe, expect, it, vi } from 'vitest';
import Track from './Track';

describe('Track (not playing)', () => {
	beforeAll(() => {
		NowPlayingStore.update({
			is_paused: false,
			is_playing: false,
			playing_track_index: -1,
			playing_tracks: [],
		});
	});

	it('renders', () => {
		render(<Track track={mockTrack1} album={mockAlbum1} />);

		expect(
			screen.getByTestId('trackContainer' + mockTrack1.file_path)
		).toBeDefined();
	});

	it('has correct CSS classes', () => {
		vi.resetModules();
		render(<Track track={mockTrack1} album={mockAlbum1} />);

		expect(
			screen.getByTestId('trackContainer' + mockTrack1.file_path).classList
		).toContain('trackContainer');
		expect(
			screen.getByTestId('trackContainer' + mockTrack1.file_path).classList
		).not.toContain('trackContainerSelected');
	});
});

describe('Track (playing)', () => {
	beforeAll(() => {
		NowPlayingStore.update({
			is_paused: false,
			is_playing: false,
			playing_track_index: 0,
			playing_tracks: [mockTrack1],
		});
	});

	it('renders', () => {
		render(<Track track={mockTrack1} album={mockAlbum1} />);

		expect(
			screen.getByTestId('trackContainer' + mockTrack1.file_path)
		).toBeDefined();
	});

	it('has correct CSS classes', () => {
		vi.resetModules();
		render(<Track track={mockTrack1} album={mockAlbum1} />);

		expect(
			screen.getByTestId('trackContainer' + mockTrack1.file_path).classList
		).toContain('trackContainer');
		expect(
			screen.getByTestId('trackContainer' + mockTrack1.file_path).classList
		).toContain('trackContainerSelected');
		vi.resetAllMocks();
		vi.resetModules();
	});
});
