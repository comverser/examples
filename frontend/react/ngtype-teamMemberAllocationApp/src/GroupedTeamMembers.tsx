import React, { useState } from "react";
import type { Dispatch } from "react";
import type { Employee } from "./App";

type GroupedTeamMembersProps = {
  employees: Employee[];
  selectedTeam: string;
  setTeam: Dispatch<React.SetStateAction<string>>;
};

const GroupedTeamMembers = ({
  employees,
  selectedTeam,
  setTeam,
}: GroupedTeamMembersProps) => {
  const [groupedEmployees, setGroupedData] = useState(groupTeamMembers());

  function groupTeamMembers() {
    var teams = [];
    var teamAMembers = employees.filter(
      (employee) => employee.teamName === "TeamA"
    );
    var teamBMembers = employees.filter(
      (employee) => employee.teamName === "TeamB"
    );
    var teamCMembers = employees.filter(
      (employee) => employee.teamName === "TeamC"
    );
    var teamDMembers = employees.filter(
      (employee) => employee.teamName === "TeamD"
    );

    var teamA = {
      team: "TeamA",
      members: teamAMembers,
      collapsed: selectedTeam === "TeamA" ? false : true,
    };
    var teamB = {
      team: "TeamB",
      members: teamBMembers,
      collapsed: selectedTeam === "TeamB" ? false : true,
    };
    var teamC = {
      team: "TeamC",
      members: teamCMembers,
      collapsed: selectedTeam === "TeamC" ? false : true,
    };
    var teamD = {
      team: "TeamD",
      members: teamDMembers,
      collapsed: selectedTeam === "TeamD" ? false : true,
    };

    teams.push(teamA);
    teams.push(teamB);
    teams.push(teamC);
    teams.push(teamD);

    return teams;
  }

  function handleTeamClick(event: React.MouseEvent<HTMLElement>) {
    var transformedGroupData = groupedEmployees.map((groupedData) =>
      groupedData.team === event.currentTarget.id
        ? { ...groupedData, collapsed: !groupedData.collapsed }
        : groupedData
    );
    setGroupedData(transformedGroupData);
    setTeam(event.currentTarget.id);
  }

  return (
    <main className="container">
      {groupedEmployees.map((item) => {
        return (
          <div
            className="card mt-2"
            key={item.team}
            style={{ cursor: "pointer" }}
          >
            <h4
              className="card-header text-secondary bg-white"
              id={item.team}
              onClick={handleTeamClick}
            >
              Team Name: {item.team}
            </h4>
            <div
              id={"collapse" + item.team}
              className={item.collapsed === true ? "collapse" : ""}
            >
              <hr />
              {item.members.map((member) => {
                return (
                  <div className="mt-2" key={member.id}>
                    <h5 className="card-title mt-2">
                      <span className="text-dark">
                        Full Name: {member.fullName}
                      </span>
                    </h5>
                    <p>Designation: {member.designation}</p>
                  </div>
                );
              })}
            </div>
          </div>
        );
      })}
    </main>
  );
};
export default GroupedTeamMembers;
