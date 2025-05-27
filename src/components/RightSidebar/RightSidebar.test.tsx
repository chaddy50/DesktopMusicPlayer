import { render, screen } from '@testing-library/react';
import { MemoryRouter } from 'react-router';
import { describe, expect, it } from 'vitest';
import RightSidebar from './RightSidebar';

describe('RightSidebar', () => {
	it('renders', () => {
		render(
			<MemoryRouter>
				<RightSidebar />
			</MemoryRouter>
		);

		expect(screen.getByTestId('rightSidebar')).toBeDefined();
	});
});
