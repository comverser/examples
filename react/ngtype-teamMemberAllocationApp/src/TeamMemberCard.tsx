import React from "react";

import type { Employee } from "./App";
import femaleProfile from "./images/femaleProfile.jpg";
import maleProfile from "./images/maleProfile.jpg";

type TeamMemberCardProps = {
    employee: Employee;
    selectedTeam: string;
    handleEmployeeCardClick: (event: React.MouseEvent<HTMLElement>) => void;
};

const TeamMemberCard = ({
    employee,
    selectedTeam,
    handleEmployeeCardClick,
}: TeamMemberCardProps) => {
    return (
        <div
            key={employee.id}
            id={employee.id.toString()}
            className={
                employee.teamName === selectedTeam
                    ? "card m-2 standout"
                    : "card m-2"
            }
            style={{ cursor: "pointer" }}
            onClick={handleEmployeeCardClick}
        >
            {employee.gender === "male" ? (
                <img src={maleProfile} className="card-img-top" />
            ) : (
                <img src={femaleProfile} className="card-img-top" />
            )}
            <div className="card-body">
                <h5 className="card-title">Full Name: {employee.fullName}</h5>
                <p className="card-text">
                    <b>Designation:</b>
                    {employee.designation}
                </p>
            </div>
        </div>
    );
};

export default TeamMemberCard;
