import React from "react";

import TeamMembers from "./TeamMembers";
import Teams from "./Teams";

import type { Employee } from "./App";

type EmployeesProps = {
    employees: Employee[];
    selectedTeam: string;
    handleEmployeeCardClick: (event: React.MouseEvent<HTMLElement>) => void;
    handleTeamSelectionChange: (
        event: React.ChangeEvent<HTMLSelectElement>
    ) => void;
};

const Employees = ({
    employees,
    selectedTeam,
    handleEmployeeCardClick,
    handleTeamSelectionChange,
}: EmployeesProps) => {
    return (
        <main className="container">
            <div className="row justify-content-center mt3-b mb-3">
                <div className="col-6">
                    <Teams
                        selectedTeam={selectedTeam}
                        handleTeamSelectionChange={handleTeamSelectionChange}
                    />
                </div>
            </div>
            <div className="row justify-content-center mt3-b mb-3">
                <div className="col-8">
                    <div className="card-collection">
                        <TeamMembers
                            employees={employees}
                            selectedTeam={selectedTeam}
                            handleEmployeeCardClick={handleEmployeeCardClick}
                        />
                    </div>
                </div>
            </div>
        </main>
    );
};

export default Employees;
