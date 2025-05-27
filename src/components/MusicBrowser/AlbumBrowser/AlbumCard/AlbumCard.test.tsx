import { mockAlbum1, mockAlbumArtist1 } from '@/tests/MockObjects';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import AlbumCard from './AlbumCard';

describe('AlbumCard (unselected)', () => {
	it('renders', () => {
		render(
			<AlbumCard
				album={mockAlbum1}
				albumArtist={mockAlbumArtist1}
				isSelected={false}
				selectAlbum={() => {}}
			/>
		);

		expect(screen.getByTestId('albumCard' + mockAlbum1.id)).toBeDefined();
		expect(
			screen.getByTestId('albumImage' + mockAlbum1.id).classList
		).toContain('albumArtworkContainer');
	});
});

describe('AlbumCard (selected)', () => {
	it('renders', () => {
		render(
			<AlbumCard
				album={mockAlbum1}
				albumArtist={mockAlbumArtist1}
				isSelected={true}
				selectAlbum={() => {}}
			/>
		);

		expect(screen.getByTestId('albumCard' + mockAlbum1.id)).toBeDefined();
		expect(
			screen.getByTestId('albumImage' + mockAlbum1.id).classList
		).toContain('albumArtworkContainerSelected');
	});
});
