import { mockAlbum1, mockAlbum2, mockAlbumArtist1 } from '@/tests/MockObjects';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import AlbumBrowser from './AlbumBrowser';

const mockAlbums = [mockAlbum1, mockAlbum2];

describe('AlbumBrowser', () => {
	it('renders all albums', () => {
		render(
			<AlbumBrowser
				albums={mockAlbums}
				albumArtist={mockAlbumArtist1}
				selectedAlbumIndex={0}
				setSelectedAlbumIndex={() => {}}
				albumListContainerRef={undefined}
			/>
		);

		expect(screen.getByTestId('albumBrowser')).toBeDefined();
		expect(screen.getByTestId('albumCard' + mockAlbum1.id)).toBeDefined();
		expect(screen.getByTestId('albumCard' + mockAlbum2.id)).toBeDefined();
	});
});
