import { mockAlbum1, mockTrack1, mockTrack2 } from '@/tests/MockObjects';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import TrackBrowser from './TrackBrowser';

describe('TrackBrowser', () => {
	it('renders all tracks', () => {
		render(<TrackBrowser album={mockAlbum1} />);

		expect(
			screen.getByTestId('trackContainer' + mockTrack1.file_path)
		).toBeDefined();
		expect(
			screen.getByTestId('trackContainer' + mockTrack2.file_path)
		).toBeDefined();
	});
});
