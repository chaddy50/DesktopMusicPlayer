import "./TabStrip.css";

interface TabProps {
    title: string;
    isSelected: boolean;
    selectTab: () => void;
}

function Tab(props: TabProps) {
    const { title, isSelected, selectTab } = props;

    return (
        <div style={{ paddingLeft: "10px", paddingRight: "10px", borderLeft: "1px solid black", borderRight: "1px solid black" }} onClick={selectTab}>
            <p className={isSelected ? "selectedTab" : "unselectedTab"}>{title}</p>
        </div>
    );
}

export default Tab;