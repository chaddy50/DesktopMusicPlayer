import GenreData from '@/dataObjects/GenreData';
import { action, makeObservable, observable } from 'mobx';

class SelectedGenreStore {
	constructor() {
		makeObservable(this, {
			genre: observable,
			update: action,
		});
	}

	genre: GenreData | undefined = undefined;

	update(newSelectedGenre: GenreData) {
		this.genre = newSelectedGenre;
	}
}

export const selectedGenreStore = new SelectedGenreStore();
