import { mockTrack1 } from '@/tests/MockObjects';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import TrackInfo from './TrackInfo';

describe('TrackInfo', () => {
	it('has correct CSS class when not playing', () => {
		render(<TrackInfo track={mockTrack1} isPlaying={false} />);

		expect(
			screen.getByTestId('trackInfo' + mockTrack1.file_path)
		).toBeDefined();
		expect(
			screen.getByTestId('trackInfo' + mockTrack1.file_path).classList
		).toContain('trackInfoContainer');
		expect(
			screen.getByTestId('trackInfo' + mockTrack1.file_path).classList
		).not.toContain('playingTrack');
	});

	it('has correct CSS class when playing', () => {
		render(<TrackInfo track={mockTrack1} isPlaying={true} />);

		expect(
			screen.getByTestId('trackInfo' + mockTrack1.file_path)
		).toBeDefined();
		expect(
			screen.getByTestId('trackInfo' + mockTrack1.file_path).classList
		).toContain('trackInfoContainer');
		expect(
			screen.getByTestId('trackInfo' + mockTrack1.file_path).classList
		).toContain('playingTrack');
	});
});
