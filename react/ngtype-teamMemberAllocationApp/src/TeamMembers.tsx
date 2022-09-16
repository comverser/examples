import React from "react";

import TeamMemberCard from "./TeamMemberCard";

import type { Employee } from "./App";

type TeamMembersProps = {
    employees: Employee[];
    selectedTeam: string;
    handleEmployeeCardClick: (event: React.MouseEvent<HTMLElement>) => void;
};

const TeamMembers = ({
    employees,
    selectedTeam,
    handleEmployeeCardClick,
}: TeamMembersProps) => {
    return (
        <>
            {employees.map((employee) => (
                <TeamMemberCard
                    key={employee.id}
                    employee={employee}
                    selectedTeam={selectedTeam}
                    handleEmployeeCardClick={handleEmployeeCardClick}
                />
            ))}
        </>
    );
};

export default TeamMembers;
