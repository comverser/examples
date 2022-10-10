import React from "react";

type TeamsProps = {
    selectedTeam: string;
    handleTeamSelectionChange: (
        event: React.ChangeEvent<HTMLSelectElement>
    ) => void;
};

const Teams = ({ selectedTeam, handleTeamSelectionChange }: TeamsProps) => {
    return (
        <select
            className="form-select form-select-lg"
            value={selectedTeam}
            onChange={handleTeamSelectionChange}
        >
            <option value="TeamA">TeamA</option>
            <option value="TeamB">TeamB</option>
            <option value="TeamC">TeamC</option>
            <option value="TeamD">TeamD</option>
        </select>
    );
};

export default Teams;
