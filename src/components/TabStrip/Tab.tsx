import "../../MusicPlayer.css";

interface TabProps {
    title: string;
    isSelected: boolean;
    selectTab: () => void;
}

function Tab(props: TabProps) {
    const { title, isSelected, selectTab } = props;

    return (
        <div className="tab" onClick={selectTab}>
            <p className={isSelected ? "selectedTab" : "unselectedTab"}>
                {title}
            </p>
        </div>
    );
}

export default Tab;
