Feature: list subfolders matching a condition

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: it displays the folders where the condition returns 0

    Scenario: happy path
      When running "m list ls go.mod"
      Then it prints:
        """
        ...

        2/3 folders match:
        1. go
        2. go_node
        """
      And it returns "success"
      And I am now in the "simple" example folder
      And there is no saved state

    @this
    Scenario: call without command
      When running "m list"
      Then it prints:
        """
        ERROR: missing condition

        The list command displays all active directories for whom the given condition returns exit code 0.
        You need to tell me which CLI command I should run in each directory to determine whether it matches.
        You do it like this:

          m list <condition>

        Example:

          m list test -f README.md
        """
      And it returns "failure"
