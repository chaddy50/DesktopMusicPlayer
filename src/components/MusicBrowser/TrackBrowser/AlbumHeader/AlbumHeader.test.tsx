import { mockAlbum1 } from '@/tests/MockObjects';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import AlbumHeader from './AlbumHeader';

describe('AlbumHeader', () => {
	it('renders', () => {
		render(<AlbumHeader albumData={mockAlbum1} />);

		expect(screen.getByTestId('albumHeaderContainer')).toBeDefined();
	});
});
