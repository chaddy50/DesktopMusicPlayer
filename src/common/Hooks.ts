import { useEffect, useState } from 'react';

export function useAlbumArtwork(artworkSource: string): string {
	if (artworkSource === 'NO_ARTWORK') {
		return '/default_cover.jpg';
	}
	return artworkSource;
}

export function useSingleAndDoubleClick(
	singleClickAction: () => void,
	doubleClickAction: () => void
) {
	const [clickCount, setClickCount] = useState(0);

	useEffect(() => {
		const timer = setTimeout(() => {
			if (clickCount === 1) {
				singleClickAction();
			}
			setClickCount(0);
		}, 200);

		if (clickCount === 2) {
			doubleClickAction();
		}

		return () => clearTimeout(timer);
	}, [clickCount, setClickCount]);

	return () => setClickCount((previous) => previous + 1);
}
