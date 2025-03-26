import { mockAlbumArtist1, mockAlbumArtist2 } from '@/tests/MockObjects';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import LeftSidebar from './LeftSidebar';

const mockAlbumArtists = [mockAlbumArtist1, mockAlbumArtist2];

describe('LeftSidebar', () => {
	it('renders', () => {
		render(
			<LeftSidebar
				albumArtists={mockAlbumArtists}
				selectedAlbumArtistIndex={0}
				setSelectedAlbumArtistIndex={() => {}}
			/>
		);

		expect(screen.getByTestId('leftSidebar')).toBeDefined();
	});
});
