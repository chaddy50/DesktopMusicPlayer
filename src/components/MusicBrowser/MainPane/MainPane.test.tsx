import { mockAlbum1, mockAlbum2, mockAlbumArtist1 } from '@/tests/MockObjects';
import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import MainPane from './MainPane';

const mockAlbums = [mockAlbum1, mockAlbum2];

describe('MainPane', () => {
	it('renders with no selected album', () => {
		render(
			<MainPane
				albums={mockAlbums}
				albumArtist={mockAlbumArtist1}
				selectedAlbumIndex={-1}
				setSelectedAlbumIndex={() => {}}
				albumListContainerRef={undefined}
			/>
		);

		expect(screen.getByTestId('mainPaneContainer')).toBeDefined();
		expect(screen.getByTestId('albumBrowser')).toBeDefined();
		expect(screen.queryByTestId('trackBrowser')).toBeNull();
	});

	it('renders with a selected album', () => {
		window.HTMLElement.prototype.scrollIntoView = vi.fn();

		render(
			<MainPane
				albums={mockAlbums}
				albumArtist={mockAlbumArtist1}
				selectedAlbumIndex={0}
				setSelectedAlbumIndex={() => {}}
				albumListContainerRef={undefined}
			/>
		);

		expect(screen.getByTestId('mainPaneContainer')).toBeDefined();
		expect(screen.getByTestId('albumBrowser')).toBeDefined();
		expect(screen.getByTestId('trackBrowser')).toBeDefined();
	});
});
