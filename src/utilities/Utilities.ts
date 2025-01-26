export function formatTimeDuration(timeDurationInSeconds: number): string {
    const hours = Math.floor(timeDurationInSeconds / 60 / 60);

    let minutes = Math.floor(timeDurationInSeconds / 60) % 60;
    let minutesAsString = "";
    if (minutes < 10 && hours > 0) {
        minutesAsString += "0";
    }
    minutesAsString += minutes;

    let seconds = timeDurationInSeconds % 60;
    let secondsAsString = "";
    if (seconds < 10) {
        secondsAsString += "0";
    }
    secondsAsString += seconds;

    if (hours > 0) {
        return `${hours}:${minutesAsString}:${secondsAsString}`;
    } else {
        return `${minutes}:${secondsAsString}`;
    }
}

export function useAlbumArtwork(artworkSource: string): string {
    if (artworkSource === "NO_ARTWORK") {
        return "/default_cover.jpg";
    }
    return artworkSource;
}
