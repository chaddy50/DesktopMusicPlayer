import { useState } from "react";
import Tab from "./Tab";

interface TabStripProps {
    tabOptions: string[],
    selectedTab: number,
    selectTab(index: number): void,
}

function TabStrip(props: TabStripProps) {
    const { tabOptions, selectedTab, selectTab } = props;

    return (
        <div style={{ display: "flex", flexDirection: "row" }}>
            {tabOptions.map((tabOption, index) => {
                return <Tab key={tabOption} title={tabOption} isSelected={index === selectedTab} selectTab={() => selectTab(index)} />
            })}
        </div>
    );
}

export default TabStrip;