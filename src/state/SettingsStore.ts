import SettingData from '@/dataObjects/SettingData';
import { action, makeObservable, observable } from 'mobx';

class SettingsStore {
	constructor() {
		makeObservable(this, {
			settings: observable,
			update: action,
		});
	}

	settings: SettingData[] = [];

	update(newSettingsData: SettingData[]) {
		console.log('newSettings data: ' + newSettingsData[0].value);
		this.settings = newSettingsData;
	}
}

export default new SettingsStore();
