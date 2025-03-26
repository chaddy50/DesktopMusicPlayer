import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import RightSidebar from './RightSidebar';

describe('RightSidebar', () => {
	it('renders', () => {
		render(<RightSidebar />);

		expect(screen.getByTestId('rightSidebar')).toBeDefined();
	});
});
