import { mockAlbumArtist1, mockAlbumArtist2 } from '@/tests/MockObjects';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import AlbumArtistBrowser from './AlbumArtistBrowser';

const mockAlbumArtists = [mockAlbumArtist1, mockAlbumArtist2];

describe('AlbumArtistBrowser', () => {
	it('renders', () => {
		render(
			<AlbumArtistBrowser
				albumArtists={[]}
				selectedAlbumArtistIndex={0}
				setSelectedAlbumArtistIndex={() => {}}
			/>
		);

		expect(screen.getByTestId('albumArtistBrowserContainer')).toBeDefined();
	});

	it('renders all album artists', () => {
		render(
			<AlbumArtistBrowser
				albumArtists={mockAlbumArtists}
				selectedAlbumArtistIndex={0}
				setSelectedAlbumArtistIndex={() => {}}
			/>
		);

		expect(screen.getByTestId('albumArtistBrowserContainer')).toBeDefined();
		expect(
			screen.getByTestId('albumArtistCard' + mockAlbumArtist1.id)
		).toBeDefined();
		expect(
			screen.getByTestId('albumArtistCard' + mockAlbumArtist2.id)
		).toBeDefined();
	});
});
