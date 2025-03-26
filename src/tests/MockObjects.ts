import AlbumArtistData from '@/dataObjects/AlbumArtistData';
import AlbumData from '@/dataObjects/AlbumData';
import TrackData from '@/dataObjects/TrackData';

export const mockTrack1: TrackData = {
	name: 'Test Track',
	album_artist_id: 1,
	album_artist_name: 'album artist',
	artist_id: 1,
	artist_name: 'artist',
	genre_id: 1,
	genre_name: 'genre',
	artwork_source: 'artwork',
	file_path: 'track_1',
	track_number: 1,
	duration_in_seconds: 1,
	album_name: 'album',
};

export const mockTrack2: TrackData = {
	name: 'Test Track 2',
	album_artist_id: 2,
	album_artist_name: 'album artist',
	artist_id: 2,
	artist_name: 'artist',
	genre_id: 2,
	genre_name: 'genre',
	artwork_source: 'artwork',
	file_path: 'track_2',
	track_number: 2,
	duration_in_seconds: 2,
	album_name: 'album',
};

export const mockAlbum1: AlbumData = {
	id: 1,
	name: 'album',
	genre_id: 1,
	genre_name: 'genre',
	album_artist_id: 1,
	album_artist_name: 'album artist',
	artwork_source: 'artwork',
	tracks: [mockTrack1, mockTrack2],
	year: 2000,
	duration_in_seconds: 1,
};

export const mockAlbum2: AlbumData = {
	id: 2,
	name: 'album 2',
	genre_id: 2,
	genre_name: 'genre 2',
	album_artist_id: 2,
	album_artist_name: 'album artist 2',
	artwork_source: 'artwork 2',
	tracks: [mockTrack2, mockTrack1],
	year: 1999,
	duration_in_seconds: 10,
};

export const mockAlbumArtist1: AlbumArtistData = {
	id: 1,
	name: 'album artist 1',
};

export const mockAlbumArtist2: AlbumArtistData = {
	id: 2,
	name: 'album artist 2',
};
