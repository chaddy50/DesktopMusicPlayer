import { mockAlbumArtist1 } from '@/tests/MockObjects';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import AlbumArtistCard from './AlbumArtistCard';

describe('AlbumArtistCard (unselected)', () => {
	it('hass correct CSS class', () => {
		render(
			<AlbumArtistCard
				albumArtist={mockAlbumArtist1}
				isSelected={false}
				selectArtist={() => {}}
			/>
		);

		expect(
			screen.getByTestId('albumArtistCard' + mockAlbumArtist1.id)
		).toBeDefined();
		expect(screen.getByText(mockAlbumArtist1.name).classList).toContain(
			'unselectedArtistCard'
		);
	});
});

describe('AlbumArtistCard (selected)', () => {
	it('has correct CSS class', () => {
		render(
			<AlbumArtistCard
				albumArtist={mockAlbumArtist1}
				isSelected={true}
				selectArtist={() => {}}
			/>
		);

		expect(screen.getByText(mockAlbumArtist1.name).classList).toContain(
			'selectedArtistCard'
		);
	});
});
